use axum::{routing::post, Router};

use self::services::{create_student, delete_student_by_id, get_all_students, get_student_by_id, update_student_by_id};

pub mod services;




pub fn get_student_route()-> Router{
    let routees = Router::new().route("/student_get_all", post(get_all_students))
    .route("/student/:id", post(get_student_by_id))
    .route("/student_delete/:id", post(delete_student_by_id))
    .route("/student_update/:id/:name/:phone/:email/:city/:address",post( update_student_by_id))
    .route("/create_student", post(create_student));
    routees
}


