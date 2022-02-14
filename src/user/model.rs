use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub departement: String,
    pub salary: i32,
    pub age: i32,
}