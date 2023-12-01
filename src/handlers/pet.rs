use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/pets")]
async fn index(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let pets: Vec<Pet> = sqlx::query_as!(Pet,"SELECT * FROM pet")
    .fetch_all(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(pets))
}

#[post("/pets")]
async fn create(pool: web::Data<PgPool>, payload: web::Json<Pet>) -> Result<HttpResponse, Error> {
    let pet_payload: Pet = payload.into_inner();

    let inserted_pet: Pet = sqlx::query_as!(Pet, "INSERT INTO pet(name, breed, age, owner, flag_removed)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING id, name, breed, age, owner, flag_removed",
    pet_payload.name,
    pet_payload.breed,
    pet_payload.age,
    pet_payload.owner,
    pet_payload.flag_removed)
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(inserted_pet))
}

#[get("/pets/{pet_id}")]
async fn select(pool: web::Data<PgPool>, pet_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as!(Pet, "SELECT * FROM pet WHERE id = $1", pet_id.into_inner())
    .fetch_optional(&**pool)
    .await;

    match result {
        Ok(Some(pet)) => Ok(HttpResponse::Ok().json(pet)),
        Ok(None) => Ok(HttpResponse::NotFound().body("Pet not found")),
        Err(e) => {
            eprintln!("Error querying database: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[patch("/pets/{pet_id}")]
async fn update(pool: web::Data<PgPool>, pet_id: web::Path<i32>, payload: web::Json<Pet>) -> Result<HttpResponse, Error> {
    let pet_payload: Pet = payload.into_inner();

    let updated_pet: Pet = sqlx::query_as!(Pet, "UPDATE pet
    SET (name, breed, age, owner, flag_removed) = ($2, $3, $4, $5, $6)
    WHERE id = $1
    RETURNING id, name, breed, age, owner, flag_removed",
    pet_id.into_inner(),
    pet_payload.name,
    pet_payload.breed,
    pet_payload.age,
    pet_payload.owner,
    pet_payload.flag_removed)
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(updated_pet))
}

#[delete("/pets/{pet_id}")]
async fn delete(pool: web::Data<PgPool>, pet_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let query_result = sqlx::query!("DELETE FROM pet WHERE id = $1", pet_id.into_inner())
        .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows_affected: u64 = query_result.rows_affected();
    Ok(HttpResponse::Ok().json(rows_affected))
}
