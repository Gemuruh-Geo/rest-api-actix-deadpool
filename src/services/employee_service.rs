use actix_web::{get, HttpResponse, Responder, web};
use deadpool_postgres::Pool;
use crate::user::model::Employee;

#[get("/employees")]
async fn get_all_employee(db: web::Data<Pool>) -> impl Responder {
    let t = db.get_ref();
    let emp_result = Employee::get_all_employee(t).await;
    match emp_result {
        Ok(employees) => HttpResponse::Ok().json(employees),
        Err(_) => HttpResponse::NotFound().body("Not Found")
    }
    
}