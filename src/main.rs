mod services;
mod user;

use actix_web::{App, HttpServer};
use services::health_check_service;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Start Server at {}:{}", "127.0.0.1",8080);
    HttpServer::new(|| {
       App::new()
        .service(health_check_service::do_health_check)
   })
   .bind(("127.0.0.1", 8080))?
   .run()
   .await
}