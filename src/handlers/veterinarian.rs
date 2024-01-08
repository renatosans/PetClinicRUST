use sqlx::postgres::PgPool;
use actix_web::{get, post, web, HttpResponse, Error};
use crate::repository::repository::Repository;
use crate::domain::veterinarian::Veterinarian;
use crate::repository::veterinarian::VeterinarianRepository;


#[get("/veterinarians/{id}")]
async fn get_vet(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let rep = VeterinarianRepository::new(&**pool);
    let veterinarian: Option<Veterinarian> = rep.get_by_id(id.into_inner())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(veterinarian))
}

#[post("/veterinarians")]
async fn post_vet(pool: web::Data<PgPool>, payload: web::Json<Veterinarian>) -> Result<HttpResponse, Error> {
    let veterinarian: Veterinarian = payload.into_inner();

    let rep = VeterinarianRepository::new(&**pool);
    let inserted = rep.insert(veterinarian)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(inserted))
}
