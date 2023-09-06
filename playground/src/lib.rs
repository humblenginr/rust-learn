use std::net::{self, TcpListener};

fn createListener(address: &str) -> Result<TcpListener, std::io::Error> {
    let listener = TcpListener::bind(address)?;
    Ok(listener)
}

// not using recursion
fn programThatPrintsItself() -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use crate::createListener;

    #[test]
    fn it_works() {
        let address = "localhost:9090";
        let listener = createListener(address).unwrap();
        let local_address: String = listener.local_addr().unwrap().to_string();
        assert_eq!(address, local_address);
    }
}
