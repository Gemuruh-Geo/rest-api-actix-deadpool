use crate::util::my_error::MyError;
use chrono:: NaiveDateTime;
use serde::{Deserialize, Serialize};
use deadpool_postgres::Pool;
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub departement: String,
    pub salary: i32,
    pub age: i32,
    pub created_on: NaiveDateTime,
}
impl Employee {
    pub async fn get_all_employee(pool: &Pool) -> Result<Vec<Employee>, MyError> {
        let conn = pool.get().await.unwrap();
        let stmt = conn.prepare_cached("SELECT * FROM employee").await.unwrap();
        let rows:Vec<Row> = conn.query(&stmt, &[]).await.unwrap();
        let employees: Vec<Employee> = rows.iter().map(|row| -> Employee {
            Employee { 
                id: row.get("id"), 
                first_name: row.get("first_name"), 
                last_name: row.get("last_name"), 
                departement: row.get("department"), 
                salary: row.get("salary"), 
                age: row.get("age"),
                created_on: row.get("created_on"),
            }
        }).collect();
        if employees.is_empty() {
            Err(MyError::NotFound)
        }else {
            Ok(employees)
        }
        
    }

    pub async fn get_emp_by_id(id: i32, pool: &Pool) -> Result<Employee, MyError> {
        let conn = pool.get().await.unwrap();
        let stmt = conn.prepare_cached("SELECT * FROM employee WHERE id = $1").await.unwrap();
        let row = conn.query_one(&stmt, &[&id]).await;
        match row {
            Ok(row) => {
            
               let e = Employee { 
                   id: row.get("id"), 
                   first_name: row.get("first_name"), 
                   last_name: row.get("last_name"), 
                   departement: row.get("department"), 
                   salary: row.get("salary"), 
                   age: row.get("age"),
                   created_on: row.get("created_on"),
                };
                Ok(e)
            },
            Err(_) => Err(MyError::NotFound)
        }
    }
}