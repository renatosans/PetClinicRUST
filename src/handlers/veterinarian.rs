// use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPool;
use actix_web::{get, post, web, HttpResponse, Error};
use crate::repository::repository::Repository;
use crate::domain::treatment::{new_treatment, Treatment};
use crate::domain::veterinarian::{check_credentials, new_veterinarian, Veterinarian};
use crate::repository::veterinarian::VeterinarianRepository;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Pet {
    id: i32
}

// TODO: implementar o use case de preescrição
#[post("/receitar_tratamento")]
async fn receitar_tratamento(_pool: web::Data<PgPool>, pet: web::Json<Pet>/*...Json<Uuid>*/ ) -> Result<HttpResponse, Error> {
    let pet: Pet = pet.into_inner();
    let veterinarian: Veterinarian = new_veterinarian("Doctor Who".to_string(), "SP 9876543210".to_string()).unwrap();
    let inscricao_crmv = veterinarian.inscricao_crmv.clone();
    let _credentials = check_credentials(inscricao_crmv).unwrap();
    let treatment: Treatment = new_treatment("antibiótico".to_string(), pet.id, veterinarian).unwrap();

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
