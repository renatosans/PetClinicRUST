use dotenv::dotenv;
// use actix_cors::Cors;
// use actix_web::{web, middleware, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Unable to load environment variables from .env file");
    let database_url: String = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    
}
