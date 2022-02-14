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
#[get("/employee/{id}")]
async fn get_employee_by_id(params: web::Path<i32>, db: web::Data<Pool>) -> impl Responder {
    let id = params.into_inner();
    let emp_result = Employee::get_emp_by_id(id, db.get_ref()).await;
    match emp_result {
        Ok(emp) => HttpResponse::Ok().json(emp),
        Err(_) => HttpResponse::NotFound().body("Not Found")
    }

}