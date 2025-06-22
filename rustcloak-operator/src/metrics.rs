use crate::error::Result;
use axum::{
    Json, Router,
    body::Body,
    http::{Response, StatusCode, header::CONTENT_TYPE},
    response::IntoResponse,
    routing::get,
    serve,
};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

pub struct Metrics {
    prom_handle: Arc<PrometheusHandle>,
    sock_addr: SocketAddr,
}

impl Metrics {
    pub fn create(sock_addr: SocketAddr) -> Result<Self> {
        let prom_handle =
            Arc::new(PrometheusBuilder::new().install_recorder()?);

        Ok(Self {
            prom_handle,
            sock_addr,
        })
    }

    async fn metrics(prom_handle: Arc<PrometheusHandle>) -> impl IntoResponse {
        prom_handle.run_upkeep();

        let body = prom_handle.render();
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/plain; version=0.0.4")
            .body(Body::from(body))
            .unwrap()
    }

    async fn health() -> impl IntoResponse {
        Json("healthy")
    }

    pub async fn run(self) -> Result<()> {
        let router = Router::new()
            .route("/healthz", get(Self::health))
            .route("/metrics", get(move || Self::metrics(self.prom_handle)));

        let listener = TcpListener::bind(self.sock_addr).await?;
        serve(listener, router)
            .with_graceful_shutdown(async {
                let _ = tokio::signal::ctrl_c().await;
            })
            .await?;

        Ok(())
    }
}
