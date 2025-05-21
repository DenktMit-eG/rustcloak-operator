use crate::error::Result;
use axum::{Json, Router, routing::get};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn serve(socket_addr: SocketAddr) -> Result<()> {
    let bind = TcpListener::bind(socket_addr).await?;
    let handle = PrometheusBuilder::new().install_recorder()?;

    let router = Router::new()
        .route("/health", get(async || Json("healthy")))
        .route("/metrics", get(async move || handle.render()));
    axum::serve(bind, router).await?;
    Ok(())
}
