use chrono::NaiveDate;

pub struct Treatment {
    pub id: Option<i32>,
    pub description: String,
    pub pet: i32,
    pub veterinarian: i32,
}
