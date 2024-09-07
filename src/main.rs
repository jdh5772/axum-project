use std::error;

use codes::{
    address::get_socket_addr,
    router::create_router,
    server::{create_tcp_listener, run_server},
};

mod codes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app = create_router();
    let addr = get_socket_addr()?;
    let listener = create_tcp_listener(addr).await?;
    run_server(listener, app).await?;
    Ok(())
}
