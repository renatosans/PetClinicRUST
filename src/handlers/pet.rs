use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/pets/{pet_id}")]
async fn select(pool: web::Data<PgPool>, pet_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let pet = web::block(move || {
        let connPool = pool.get_ref();

        let result = sqlx::query_as!(Pet,"SELECT * FROM pet WHERE id = $1", pet_id.into_inner())
        .fetch_one(connPool);

        return result;
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(pet.await.unwrap()))
}

#[patch("/pets/{pet_id}")]
async fn update(pool: web::Data<PgPool>, pet_id: web::Path<i32>, payload: web::Json<Pet>) -> Result<HttpResponse, Error> {
    let car = web::block(move || {
        let result: Result<usize, Error> = diesel::update(pet.find(pet_id.into_inner())).set(payload.into_inner()).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}

#[delete("/pets/{pet_id}")]
async fn delete(pool: web::Data<PgPool>, pet_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let query_result = web::block(move || {
        let connPool = pool.get_ref();

        let result = sqlx::query!("DELETE FROM pet WHERE id = $1", pet_id.into_inner())
        .execute(connPool);

        return result;
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows_affected = query_result.await.unwrap().rows_affected();
    Ok(HttpResponse::Ok().json(rows_affected))
}
