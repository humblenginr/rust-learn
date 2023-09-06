use std::net::Ipv4Addr;

use libarp::client::ArpClient;

fn main() {
    let mut arp_client = ArpClient::new().expect("Cannot create ARP Client");
    // let arp_req = ArpMessage::new_arp_request(
    //     MacAddr::new(188, 208, 116, 72, 138, 98),
    //     Ipv4Addr::new(172, 17, 61, 80),
    //     Ipv4Addr::new(172, 17, 14, 237),
    // );
    let mac_addr = arp_client
        .ip_to_mac(Ipv4Addr::new(172, 17, 14, 237), None)
        .expect("ARP Request unsuccesful");
    println!("Mac: {}", mac_addr.to_string());
}
