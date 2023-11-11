use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/owners/{owner_id}")]
async fn select(pool: web::Data<PgPool>, owner_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let car = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<Option<Petowner>, Error> = cars_for_sale.find(car_id.into_inner()).first(&mut conn).optional();
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}

#[patch("/owners/{owner_id}")]
async fn update(pool: web::Data<PgPool>, owner_id: web::Path<i32>, payload: web::Json<Petowner>) -> Result<HttpResponse, Error> {
    let car = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, Error> = diesel::update(cars_for_sale.find(car_id.into_inner())).set(payload.into_inner()).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}

#[delete("/owners/{owner_id}")]
async fn delete(pool: web::Data<PgPool>, owner_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let connPool = pool.get_ref();

    let query_result = web::block(move || {

        let result = sqlx::query!("DELETE FROM petowner WHERE id = $1", owner_id.into_inner())
        .execute(connPool);

        return result;
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows_affected = query_result.await.unwrap().rows_affected();
    Ok(HttpResponse::Ok().json(rows_affected))
}
