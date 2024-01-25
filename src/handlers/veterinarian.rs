// use uuid::Uuid;
use sqlx::postgres::PgPool;
use actix_web::{get, post, web, HttpResponse, Error};
use crate::repository::repository::Repository;
use crate::domain::treatment::{new_treatment, Treatment};
use crate::domain::veterinarian::{new_veterinarian, Veterinarian};
use crate::repository::veterinarian::VeterinarianRepository;


// TODO: implementar o use case de preescrição
#[post("/receitar_tratamento")]
async fn receitar_tratamento(pool: web::Data<PgPool>, pet: web::Json<i32>/*...Json<Uuid>*/ ) -> Result<HttpResponse, Error> {
    let veterinarian: Veterinarian = new_veterinarian("Doctor Who".to_string(), "SP 9876543210".to_string()).unwrap();
    let treatment: Treatment = new_treatment("antibiótico".to_string(), pet.into_inner(), veterinarian).unwrap();

    Ok(HttpResponse::Ok().json(treatment))
}

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
