// get employee Routes .

use axum::{routing::post, Router};
pub mod services;

use services::{get_all_employee, get_employee_by_id, delete_employee_by_id, update_employee, create_employee};

pub fn get_employee_route()-> Router  {

    let routees = Router::new().route("/employee_get_all", post(get_all_employee))
    .route("/employee/:id", post(get_employee_by_id))
    .route("/employee_delete/:id", post(delete_employee_by_id))
        .route("/employee_update",post( update_employee))
        .route("/employee_create", post(create_employee));
        routees
    
    }
    