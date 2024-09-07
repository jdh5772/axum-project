use std::{error, net::SocketAddr};

pub fn get_socket_addr() -> Result<SocketAddr, Box<dyn error::Error>> {
    let addr = "0.0.0.0:3000".parse::<SocketAddr>()?;
    Ok(addr)
}
