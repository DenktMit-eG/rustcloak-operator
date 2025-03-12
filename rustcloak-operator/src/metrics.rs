use actix_web::{HttpRequest, HttpResponse, Responder, get};
use prometheus::{Encoder, TextEncoder};

#[get("/metrics")]
pub async fn metrics(_: HttpRequest) -> impl Responder {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .insert_header(("Content-Type", encoder.format_type()))
        .body(buffer)
}
