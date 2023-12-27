use chrono::NaiveDate;

#[derive(Default, Debug)]
pub struct Treatment {
    pub id: Option<i32>,
    pub description: String,
    pub pet: i32,
    pub veterinarian: i32,
}

impl Treatment {
    pub fn validate(&self) -> bool {
        
        true
    }
}
