use axum::Router;
use std::{error, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn create_tcp_listener(addr: SocketAddr) -> Result<TcpListener, Box<dyn error::Error>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    Ok(listener)
}

pub async fn run_server(listener: TcpListener, app: Router) -> Result<(), Box<dyn error::Error>> {
    axum::serve(listener, app).await?;
    Ok(())
}
