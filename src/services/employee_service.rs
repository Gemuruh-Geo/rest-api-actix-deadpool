use std::vec;

use actix_web::{get, HttpResponse, Responder};

use crate::user::model::Employee;

#[get("/employees")]
async fn get_all_employee() -> impl Responder {
    let employees = vec![
        Employee {
            id: 1,
            age: 34,
            departement: String::from("IT"),
            first_name: String::from("Gemuruh"),
            last_name: String::from("Geo Pratama"),
            salary: 16,   
        },
        Employee {
            id: 2,
            age: 34,
            departement: String::from("IT"),
            first_name: String::from("Gemuruh 2"),
            last_name: String::from("Geo Pratama 2"),
            salary: 16,   
        }
    ];
    HttpResponse::Ok().json(employees)
}