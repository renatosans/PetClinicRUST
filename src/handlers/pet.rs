use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


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
    let query_result = sqlx::query!("DELETE FROM pet WHERE id = $1", pet_id.into_inner())
        .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows_affected = query_result.rows_affected();
    Ok(HttpResponse::Ok().json(rows_affected))
}
