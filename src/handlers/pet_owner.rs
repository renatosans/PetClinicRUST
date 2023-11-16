use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/owners")]
async fn index(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let owners: Vec<Petowner> = sqlx::query_as!(Petowner,"SELECT * FROM petowner")
    .fetch_all(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(owners))
}

#[post("/owners")]
async fn create(pool: web::Data<PgPool>, payload: web::Json<Petowner>) -> Result<HttpResponse, Error> {
    let owner_payload: Petowner = payload.into_inner();

    let inserted_owner: Petowner = sqlx::query_as!(Petowner, "INSERT INTO petowner(name, birth_date, email, phone, address)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING id, name, birth_date, email, phone, address",
    owner_payload.name,
    owner_payload.birth_date,
    owner_payload.email,
    owner_payload.phone,
    owner_payload.address)
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(inserted_owner))
}

#[get("/owners/{owner_id}")]
async fn select(pool: web::Data<PgPool>, owner_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let owner: Petowner = sqlx::query_as!(Petowner,"SELECT * FROM petowner WHERE id = $1", owner_id.into_inner())
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(owner))
}

#[patch("/owners/{owner_id}")]
async fn update(pool: web::Data<PgPool>, owner_id: web::Path<i32>, payload: web::Json<Petowner>) -> Result<HttpResponse, Error> {
    let owner_payload: Petowner = payload.into_inner();

    let updated_owner: Petowner = sqlx::query_as!(Petowner, "UPDATE petowner
    SET (name, birth_date, email, phone, address) = ($2, $3, $4, $5, $6)
    WHERE id = $1
    RETURNING id, name, birth_date, email, phone, address",
    owner_id.into_inner(),
    owner_payload.name,
    owner_payload.birth_date,
    owner_payload.email,
    owner_payload.phone,
    owner_payload.address)
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(updated_owner))
}

#[delete("/owners/{owner_id}")]
async fn delete(pool: web::Data<PgPool>, owner_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let query_result = sqlx::query!("DELETE FROM petowner WHERE id = $1", owner_id.into_inner())
    .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows_affected = query_result.rows_affected();
    Ok(HttpResponse::Ok().json(rows_affected))
}
