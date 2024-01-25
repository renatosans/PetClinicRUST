// use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::utils::domain_error::DomainError;
use crate::domain::veterinarian::Veterinarian;


#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Treatment {
    pub id: Option<i32>,
    pub description: String,
    pub pet: i32,
    pub veterinarian: Veterinarian,
}

pub fn new_treatment(description: String, pet: i32, veterinarian: Veterinarian) -> Result<Treatment, DomainError> {
    // let uuid = Some(Uuid::new_v4());

    let obj = Treatment {
        id: Some(9999),          // gerar UUID
        description: description,
        pet: pet,
        veterinarian: veterinarian,
    };

    if !obj.validate() {
        return Err(DomainError::ValidationError);
    }
    Ok(obj)
}

impl Treatment {
    pub fn validate(&self) -> bool {
        
        true
    }
}
