mod schema;
mod models;

use crate::schema::pet::dsl::*;
use crate::models::{Pet, /*Petowner, Veterinarian*/};

use dotenv::dotenv;
use postgres::{ Client, NoTls };
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };


#[macro_use]
extern crate serde_derive;

//Model: USer struct with id, name, email
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

//main function
fn main() {
    dotenv().expect("Unable to load environment variables from .env file");
    let database_url: String = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    //Set database
    let mut client: Client = Client::connect(database_url.clone().as_str(), NoTls).expect("Failed to connect to database");
    let _batch_result = client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users(
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL)")
    .expect("Failed to create database table");

    //start server and print port
    let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(&database_url, pool.clone(), stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

//handle_client function
fn handle_client(database_url: &str, pool: DbPool, mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post_request(database_url, r),
                r if r.starts_with("GET /users/") => handle_get_request(database_url, r),
                r if r.starts_with("GET /users") => handle_get_all_request(pool, r),
                r if r.starts_with("PUT /users/") => handle_put_request(database_url, r),
                r if r.starts_with("DELETE /users/") => handle_delete_request(database_url, r),
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
fn handle_post_request(database_url: &str, request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(database_url, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO users (name, email) VALUES ($1, $2)",
                    &[&user.name, &user.email]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_get_request function
fn handle_get_request(database_url: &str, request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(database_url, NoTls)) {
        (Ok(user_id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&user_id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_get_all_request function
fn handle_get_all_request(pool: DbPool, _request: &str) -> (String, String) {

    let mut conn = pool.get().unwrap(); // TODO: fix unwrap
    let result: Result<Vec<Pet>, diesel::result::Error> = pet.load::<Pet>(&mut conn);

    std::process::Command::new("clear").status().unwrap();
    let pets = result.unwrap();
    for pet_friend in pets.iter() {
        println!("{:?}", pet_friend);
    }

    (String::from("200"), String::from("ok"))
}

//handle_put_request function
fn handle_put_request(database_url: &str, request: &str) -> (String, String) {
    match
        (
            get_id(&request).parse::<i32>(),
            get_user_request_body(&request),
            Client::connect(database_url, NoTls),
        )
    {
        (Ok(user_id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &user_id]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_delete_request function
fn handle_delete_request(database_url: &str, request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(database_url, NoTls)) {
        (Ok(user_id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&user_id]).unwrap();

            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//get_id function
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user from request body with the id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
