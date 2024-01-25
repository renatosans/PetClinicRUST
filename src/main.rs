mod utils;
mod schema;
mod models;
mod domain;
mod repository;
mod handlers;

use handlers::pet;
use handlers::pet_owner;
use actix_cors::Cors;
use actix_web_prometheus::{PrometheusMetrics, PrometheusMetricsBuilder};
use actix_web::{web, middleware, App, HttpServer};
// use diesel::prelude::*;                       // diesel ORM
use sqlx::postgres::{PgPool, PgPoolOptions};     // sqlx


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // environment vars passadas via docker compose,  remover o comentário para rodar localmente
    // dotenv().expect("Unable to load environment variables from .env file");
    let database_url: String = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool_options = PgPoolOptions::new().max_connections(100);
    let pool: PgPool = pool_options.connect(&database_url)
        .await
        .expect("Unable to connect to database");

    HttpServer::new(move || {
        let cors: Cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        // TODO: adicionar observabilidade/métricas
        let prometheus: PrometheusMetrics = PrometheusMetricsBuilder::new("api").endpoint("/metrics").build().unwrap();

        App::new()
            .wrap(cors)
            .wrap(prometheus)
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(
                web::scope("/api")
                    .service(pet::index)
                    .service(pet::create)
                    .service(pet::select)
                    .service(pet::update)
                    .service(pet::delete)
                    .service(pet_owner::index)
                    .service(pet_owner::create)
                    .service(pet_owner::select)
                    .service(pet_owner::update)
                    .service(pet_owner::delete)
                    .service(handlers::veterinarian::post_vet)
                    .service(handlers::veterinarian::receitar_tratamento)
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
