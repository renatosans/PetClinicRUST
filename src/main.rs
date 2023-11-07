use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::{web, middleware, App, HttpServer};
// use diesel::prelude::*;                       // diesel ORM
use sqlx::postgres::{PgPool, PgPoolOptions};     // sqlx


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Unable to load environment variables from .env file");
    let database_url: String = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool_options = PgPoolOptions::new().max_connections(100);
    let pool: PgPool = rt.block_on(pool_options.connect(&database_url)).expect("Unable to connect to database");

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(
                web::scope("/api")
                    .service(cars_for_sale::index)
                    .service(cars_for_sale::select)
                    .service(cars_for_sale::create)
                    .service(cars_for_sale::update)
                    .service(cars_for_sale::delete)
                    .service(salesperson::delete)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
