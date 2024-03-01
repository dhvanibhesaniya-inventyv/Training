
use axum::{extract::Path,response::{IntoResponse, Response}, Json};


use crate::utils::db_config;

use super::model::{Message, Student,allidstd};


pub async fn get_student_data() -> Response {
 
    let start_key = "".to_string();
    let end_key = "".to_string(); 

   
    let data = match db_config::scan_data(start_key, end_key).await {
        Ok(data) => data,
        Err(_) => {
            
            let message = Message {
                status: 500,
                message_key: String::from("error"),
                data: vec!["Failed to retrieve student data from TiKV".to_string()],
            };
            return Json(message).into_response()
        }
    };

  
let values: Vec<String> = data.into_iter()
.map(|(_, value)| String::from_utf8(value).unwrap())
.collect();

    
    let message = Message {
        status: 200,
        message_key: String::from("success"),
        data: values,
    };

   
    Json(message).into_response()
}






pub async fn get_student_id(Path(id): Path<i32>) -> Response {
    if let Ok(student_data) = db_config::get_data(format!("student_{}", id)).await {

        let student_vec : Student  = serde_json::from_str(&student_data).unwrap();
        let message = Message {
            status: 200,
            message_key: String::from("success"),
            data: student_vec,
        };
        Json(message).into_response()
    } else {
        let message = Message {
            status: 404,
            message_key: String::from("error"),
            data: "No data found for the given student ID".to_string(),
        };
        Json(message).into_response()
    }
}

pub async fn delete_student_id(Path(id):Path<i32>)-> Response{
    db_config::delete_data(format!("student_{}", id)).await;


    let message = Message {
        status: 200,
        message_key: String::from("success"),
        data: "student data deleted successfully".to_string(),
    };
    Json(message).into_response()

}


pub async fn update_student_id(Path(id):Path<i32>,Json(new_student): Json<Student>)-> Response{
    let got_student_data = serde_json::to_string(&new_student).expect("err");

   if let Ok(_data) = db_config::put_data(format!("student_{}", id),got_student_data).await{

        let message = Message {
            status: 200,
            message_key: String::from("successfully updated the data you can check it by getting the data through its id."),
            data:new_student,
        };


        Json(message).into_response()


    } else {
        let message = Message {
            status: 404,
            message_key: String::from("error"),
            data: "No data found for the given student ID".to_string(),
        };
        Json(message).into_response()
    }

}



pub async fn create_student(Json(new_student): Json<Student>) -> Response {
   
    let mut allid = allidstd();
    let new_id = format!("student_{}", allid.len() + 1);
    
    println!("{}",allid.len());
println!("{}",new_id);

allid.push(new_id.clone());
println!("{}",allid.len());

   
    let got_student_data = serde_json::to_string(&new_student).expect("Failed to serialize new student data");

   
    if let Ok(data) = db_config::put_data(new_id, got_student_data).await {
      
        let message = Message {
            status: 200,
            message_key: data,
            data: new_student,
        };
        Json(message).into_response()
    } else {
        
        let message = Message {
            status: 400, 
            message_key: String::from("error"),
            data: "Failed to store data in the database".to_string(),
        };
        Json(message).into_response()
    }
}