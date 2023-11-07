// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
// use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub breed: Option<String>,
    pub age: Option<i32>,
    pub owner: i32,
}

#[derive(Debug)]
pub struct Petowner {
    pub id: i32,
    pub name: String,
    pub birth_date: Option<NaiveDate>,
    pub email: String,
    pub phone: Option<String>,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Treatment {
    pub id: i32,
    pub description: String,
    pub pet: i32,
    pub veterinarian: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vaccination {
    pub id: i32,
    pub description: String,
    pub pet: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Veterinarian {
    pub id: i32,
    pub name: String,
    pub inscricao_crmv: String,
}
