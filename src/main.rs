mod services;
mod user;
mod util;
use services::health_check_service::{do_health_check};
use services::employee_service::{get_all_employee,get_employee_by_id};
use actix_web::{App, HttpServer,web};
use deadpool_postgres::{Config, RecyclingMethod, ManagerConfig, Runtime, PoolConfig};
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut cfg = Config::new();
    cfg.user = Some(String::from("actix"));
    cfg.password = Some(String::from("actix"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5431);
    cfg.dbname = Some(String::from("actix"));
    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    cfg.manager = Some(manager_config);
    let pool_config = PoolConfig::new(10);
    cfg.pool = Some(pool_config);
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    println!("Start Server at {}:{}", "127.0.0.1",8080);
    HttpServer::new(move || {
       App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(do_health_check)
        .service(get_all_employee)
        .service(get_employee_by_id)
   })
   .bind(("127.0.0.1", 8080))?
   .run()
   .await
}

