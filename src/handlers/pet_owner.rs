use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/owners/{owner_id}")]
async fn select(pool: web::Data<PgPool>, car_id: web::Path<i32>) -> Result<HttpResponse, Error> {
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
async fn update(pool: web::Data<PgPool>, car_id: web::Path<i32>, payload: web::Json<Petowner>) -> Result<HttpResponse, Error> {
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
async fn delete(pool: web::Data<PgPool>, car_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let car = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, Error> = diesel::delete(cars_for_sale.find(car_id.into_inner())).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}
