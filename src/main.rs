mod schema;
mod models;

use crate::models::{/*Pet, Petowner,*/ Veterinarian};

use dotenv::dotenv;
use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
use actix_web::{web};
use serde::{Serialize, Deserialize};
// use diesel::prelude::*;                       // diesel ORM
use sqlx::postgres::{PgPool, PgPoolOptions};     // sqlx


// model: User struct
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";


fn main() {
    dotenv().expect("Unable to load environment variables from .env file");
    let database_url: String = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool_options = PgPoolOptions::new().max_connections(100);
    let pool = rt.block_on(pool_options.connect(&database_url)).expect("Unable to connect to database");

    // set database
    let _query_result = rt.block_on(sqlx::query!(
        "CREATE TABLE IF NOT EXISTS users(
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL)")
    .execute(&pool));

    //start server and print port
    let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                rt.block_on(handle_client(&database_url, pool.clone(), stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

//handle_client function
async fn handle_client(database_url: &str, pool: PgPool, mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post_request(pool, r).await,
                r if r.starts_with("GET /users/") => handle_get_request(pool, r).await,
                r if r.starts_with("GET /users") => handle_get_all_request(pool, r).await,
                r if r.starts_with("PUT /users/") => handle_put_request(pool, r).await,
                r if r.starts_with("DELETE /users/") => handle_delete_request(pool, r).await,
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

//CONTROLLERS

//handle_post_request function
async fn handle_post_request(pool: PgPool, request: &str) -> (String, String) {
/*
    let payload: web::Json<Veterinarian> = web::Json(request);
    .execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email]
    )
*/
    (String::from("200"), String::from("ok"))
}

//handle_get_request function
async fn handle_get_request(pool: PgPool, request: &str) -> (String, String) {

    let vet_id = get_id(&request).parse::<i32>().unwrap();
    let vet: Veterinarian = sqlx::query_as!(Veterinarian,"SELECT id, name, \"inscricaoCRMV\" as inscricao_crmv FROM veterinarian WHERE id = $1", vet_id)
    .fetch_one(&pool)
    .await.expect("Unable to query database table");

    println!("{:?}", vet);

    (String::from("200"), String::from("ok"))
}

//handle_get_all_request function
async fn handle_get_all_request(pool: PgPool, _request: &str) -> (String, String) {

    let vets: Vec<Veterinarian> = sqlx::query_as!(Veterinarian,"SELECT id, name, \"inscricaoCRMV\" as inscricao_crmv FROM veterinarian")
    .fetch_all(&pool)
    .await.expect("Unable to query database table");

    for veterinarian in vets.iter() {
        println!("{:?}", veterinarian);
    }

    (String::from("200"), String::from("ok"))
}

//handle_put_request function
async fn handle_put_request(pool: PgPool, request: &str) -> (String, String) {
/*
    let vet_id = get_id(&request).parse::<i32>().unwrap();
    let payload: web::Json<Veterinarian> = web::Json(request);
    .execute(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        &[&user.name, &user.email, &user_id]
    )
*/
    (String::from("200"), String::from("ok"))
}

//handle_delete_request function
async fn handle_delete_request(pool: PgPool, request: &str) -> (String, String) {

    let vet_id = get_id(&request).parse::<i32>().unwrap();
    let query_result = sqlx::query!("DELETE FROM veterinarian WHERE id = $1", vet_id)
    .execute(&pool)
    .await.expect("Unable to query database table");

    if query_result.rows_affected() == 0 {
    }

    (String::from("200"), String::from("ok"))
}

//get_id function
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}
