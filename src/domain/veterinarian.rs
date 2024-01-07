use crate::utils::domain_error::DomainError;


#[derive(Default, Debug)]
pub struct Veterinarian {
    pub id: Option<i32>,
    pub name: String,
    pub inscricao_crmv: String,
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

impl Veterinarian {
    pub fn validate(&self) -> bool {

        true
    }
}
