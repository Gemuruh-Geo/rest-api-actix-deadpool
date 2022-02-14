mod services;
mod user;
use services::health_check_service::{do_health_check};
use services::employee_service::get_all_employee;
use actix_web::{App, HttpServer};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Start Server at {}:{}", "127.0.0.1",8080);
    HttpServer::new(|| {
       App::new()
        .service(do_health_check)
        .service(get_all_employee)
   })
   .bind(("127.0.0.1", 8080))?
   .run()
   .await
}