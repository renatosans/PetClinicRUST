use std::sync::Arc;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::utils::domain_error::DomainError;


#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Veterinarian {
    pub id: Option<i32>,
    pub name: String,
    pub inscricao_crmv: String,
}

#[derive(Default)]
pub struct Credentials {
    pub registro: String,
    pub data_inscricao: NaiveDate
}

pub fn new_veterinarian(name: String, inscricao_crmv: String) -> Result<Veterinarian, DomainError> {
    let obj = Veterinarian{
        id: None, // alterar o tipo para Option<String>, gerar UUID
        name,
        inscricao_crmv,
    };

    if !obj.validate() {
        return Err(DomainError::ValidationError);
    }
    Ok(obj)
}

// TODO: implementar a verificação (inscrito no conselho regional)
pub fn check_credentials(_inscricao_crmv: String) -> Result<Credentials, DomainError> {
    let veterinarian_list: Arc<[Veterinarian]> = vec![Veterinarian::default()].into();

    if !veterinarian_list.contains(&Veterinarian::default()) {
        return Err(DomainError::NotFoundError);
    }

    Ok(Credentials::default())
}

impl Veterinarian {
    pub fn validate(&self) -> bool {

        true
    }
}
