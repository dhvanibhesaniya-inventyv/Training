use axum::Router;


use crate::Axum_server::Users::{get_student_route,get_employee_route,get_master_route};
use crate::Axum_server::health_check::get_status_routes;


pub fn get_routes() -> Router { 
    let app = Router::new().merge(get_status_routes()).merge(get_student_route().merge(get_employee_route()).merge(get_master_route()));
    app
}