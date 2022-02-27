use std::net::SocketAddr;

use anyhow::Result;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::NamedService;
use tonic_health::ServingStatus;
use tracing::{error, info};
use warp::Filter;

async fn connect(port: u64) -> Result<(TcpListenerStream, SocketAddr)> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let addr = listener.local_addr()?;
    Ok((TcpListenerStream::new(listener), addr))
}

#[tokio::main]
async fn main() -> Result<()> {
    let service_port = option_env!("SERVICE_PORT").unwrap_or("10356").parse()?;
    let liveness_port = option_env!("LIVENESS_PORT").unwrap_or("10357").parse()?;
    let (service_stream, service_addr) = connect(service_port).await?;
    let (liveness_stream, liveness_addr) = connect(liveness_port).await?;

    let (mut health_reporter, health_server) = tonic_health::server::health_reporter();
    let health_server_name = health_server.name();

    let service = tonic::transport::Server::builder()
        .add_service(health_server)
        .serve_with_incoming(service_stream);

    let liveness = warp::path("liveness").map(warp::reply);
    let liveness = warp::serve(liveness).run_incoming(liveness_stream);

    info!(message = "Starting Engula server...", %service_addr, %liveness_addr);

    health_reporter
        .set_service_status(health_server_name, ServingStatus::Serving)
        .await;

    tokio::select! {
        res = service => {
            match res {
                Ok(()) => info!("Services are stopping..."),
                Err(err) => error!(cause = %err, "Fatal error occurs!"),
            };
        }
        _ = liveness => {
            info!("Liveness endpoint is stopping...");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Shutting down...");
        }
    }

    Ok(())
}
