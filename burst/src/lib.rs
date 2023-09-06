#![allow(dead_code, unused_variables, unused_assignments)]
use std::{collections::HashMap, io};

use rusoto_ec2::{
    DescribeInstancesRequest, DescribeSpotInstanceRequestsRequest, Ec2, RequestSpotInstancesRequest,
};

pub struct SshConnection {}
pub struct Machine {
    private_ip: String,
    public_dns: String,
    ssh: Option<SshConnection>,
    instance_type: String,
}

pub struct MachineSetup {
    instance_type: String,
    ami: String,
    setup: Box<dyn Fn(&mut SshConnection) -> Result<(), io::Error>>,
}

impl MachineSetup {
    pub fn new<F>(instance_type: String, ami: String, setup: F) -> Self
    where
        F: Fn(&mut SshConnection) -> Result<(), io::Error> + 'static,
    {
        Self {
            instance_type,
            ami,
            setup: Box::new(setup),
        }
    }
}

struct BurstBuilder {
    machine_descriptors: HashMap<String, (MachineSetup, u32)>,
    max_duration: i64,
}
impl Default for BurstBuilder {
    fn default() -> Self {
        Self {
            // uses the default value of the type
            machine_descriptors: Default::default(),
            max_duration: 1,
        }
    }
}
impl BurstBuilder {
    pub fn add_machine(&mut self, name: String, count: u32, descriptor: MachineSetup) {
        self.machine_descriptors.insert(name, (descriptor, count));
    }
    pub fn set_max_duration(&mut self, hours: u8) {
        self.max_duration = i64::from(hours) * 60;
    }
    // we are moving `self` here since we will only be running it once
    pub async fn run<R>(self, runner: R)
    where
        R: FnOnce(HashMap<String, &mut [Machine]>) -> Result<(), io::Error>,
    {
        // 1. Issue spot requests
        let mut spot_intance_req_ids = Vec::new();
        let ec2 = rusoto_ec2::Ec2Client::new(rusoto_core::Region::UsEast1);
        for (name, (setup, count)) in self.machine_descriptors {
            let mut req = RequestSpotInstancesRequest::default();
            req.instance_count = Some(i64::from(count));
            let mut launch_specification = rusoto_ec2::RequestSpotLaunchSpecification::default();
            launch_specification.image_id = Some(setup.ami);
            launch_specification.instance_type = Some(setup.instance_type);
            req.launch_specification = Some(launch_specification);
            req.block_duration_minutes = Some(self.max_duration);
            let instances = ec2
                .request_spot_instances(req)
                .await
                .unwrap()
                .spot_instance_requests
                .unwrap();
            spot_intance_req_ids.extend(
                instances
                    .into_iter()
                    .filter_map(|sir| sir.spot_instance_request_id),
            )
        }
        // 2. wait for the instances to come up
        let spot_instance_ids: Vec<String>;
        let mut describe_spot_instances_req = DescribeSpotInstanceRequestsRequest::default();
        describe_spot_instances_req.spot_instance_request_ids = Some(spot_intance_req_ids);
        loop {
            let req_descriptions = ec2
                .describe_spot_instance_requests(describe_spot_instances_req.clone())
                .await
                .unwrap()
                .spot_instance_requests
                .unwrap();
            let any_open = (&req_descriptions)
                .iter()
                .any(|srd| srd.state.as_ref().unwrap() == "open");
            if !any_open {
                spot_instance_ids = req_descriptions
                    .into_iter()
                    .map(|srd| srd.instance_id.unwrap())
                    .collect();
                break;
            }
        }
        //  3. stop spot requests
        let mut cancel = rusoto_ec2::CancelSpotInstanceRequestsRequest::default();
        cancel.spot_instance_request_ids = describe_spot_instances_req
            .spot_instance_request_ids
            .take()
            .unwrap();
        ec2.cancel_spot_instance_requests(cancel).await.unwrap();
        //  - once an instance is ready, run the setup closure
        //  4. wait until all the instances are ready and the setups have been run
        let mut machines: Vec<Machine> = Vec::new();
        let mut any_not_ready = true;
        while any_not_ready {
            any_not_ready = false;
            machines.clear();
            let mut inst_req = DescribeInstancesRequest::default();
            inst_req.instance_ids = Some(spot_instance_ids.clone());
            for reservation in ec2
                .describe_instances(inst_req)
                .await
                .unwrap()
                .reservations
                .unwrap()
            {
                for instance in reservation.instances.unwrap() {
                    match instance {
                        rusoto_ec2::Instance {
                            public_dns_name: Some(public_dns),
                            private_ip_address: Some(private_ip),
                            instance_type: Some(instance_type),
                            ..
                        } => machines.push(Machine {
                            public_dns,
                            private_ip,
                            ssh: None,
                            instance_type,
                        }),
                        _ => any_not_ready = true,
                    }
                }
            }
        }

        //  5. invoke runner(R) with machine descriptors
        //  6. tear down all the instances
    }
}
