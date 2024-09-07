use std::{error, net::SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app = create_router();
    let addr = get_socket_addr()?;
    let listener = create_tcp_listener(addr).await?;
    run_server(listener, app).await?;
    Ok(())
}

fn create_router() -> Router {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}

fn get_socket_addr() -> Result<SocketAddr, Box<dyn error::Error>> {
    let addr = "0.0.0.0:3000".parse::<SocketAddr>()?;
    Ok(addr)
}

async fn create_tcp_listener(addr: SocketAddr) -> Result<TcpListener, Box<dyn error::Error>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    Ok(listener)
}

async fn run_server(listener: TcpListener, app: Router) -> Result<(), Box<dyn error::Error>> {
    axum::serve(listener, app).await?;
    Ok(())
}
