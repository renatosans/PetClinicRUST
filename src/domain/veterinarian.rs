use chrono::NaiveDate;

#[derive(Default, Debug)]
pub struct Veterinarian {
    pub id: Option<i32>,
    pub name: String,
    pub inscricao_crmv: String,
}

impl Veterinarian {
    pub fn validate(&self) -> bool {

        true
    }
}
