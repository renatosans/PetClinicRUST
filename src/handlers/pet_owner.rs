use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/owners/{owner_id}")]
async fn select(pool: web::Data<PgPool>, owner_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let owner = web::block(move || {
        let connPool = pool.get_ref();

        let result = sqlx::query_as!(Petowner,"SELECT * FROM petowner WHERE id = $1", owner_id.into_inner())
        .fetch_one(connPool);

        return result;
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(owner.await.unwrap()))
}

#[patch("/owners/{owner_id}")]
async fn update(pool: web::Data<PgPool>, owner_id: web::Path<i32>, payload: web::Json<Petowner>) -> Result<HttpResponse, Error> {
    let owner = web::block(move || {
        let connPool = pool.get_ref();

        let owner_payload: Petowner = payload.into_inner();

        let updated_owner = sqlx::query_as!(Petowner, "UPDATE petowner
        SET (name, birth_date, email, phone, address) = ($2, $3, $4, $5, $6)
        WHERE id = $1
        RETURNING id, name, birth_date, email, phone, address",
        owner_id.into_inner(),
        owner_payload.name,
        owner_payload.birth_date,
        owner_payload.email,
        owner_payload.phone,
        owner_payload.address)
        .fetch_one(connPool);

        return result;
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(owner))
}

#[delete("/owners/{owner_id}")]
async fn delete(pool: web::Data<PgPool>, owner_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let query_result = web::block(move || {
        let connPool = pool.get_ref();

        let result = sqlx::query!("DELETE FROM petowner WHERE id = $1", owner_id.into_inner())
        .execute(connPool);

        return result;
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows_affected = query_result.await.unwrap().rows_affected();
    Ok(HttpResponse::Ok().json(rows_affected))
}
