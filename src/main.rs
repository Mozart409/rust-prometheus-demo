use std::collections::HashMap;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

async fn health() -> HttpResponse {
    // return 200 OK with "OK" in the body
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

    log::info!("starting HTTP server at http://localhost:3000");
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/health").to(health))
            .service(web::resource("/").to(health))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await?;
    Ok(())
}
