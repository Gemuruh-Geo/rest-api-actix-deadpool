use actix_web::{ResponseError, HttpResponse};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use std::fmt::Debug;

#[derive(Display, From, Debug)]
pub enum MyError {
    PoolError(PoolError),
    NotFound,
}

impl ResponseError for MyError {

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
           
        }
    }


}