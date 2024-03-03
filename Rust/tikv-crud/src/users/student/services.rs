
use axum::{extract::Path,response::{IntoResponse, Response}, Json};


use crate::utils::db_config;

use super::model::{Message, Student,allidstd};


pub async fn get_student_data() -> Response {

    let allids = allidstd();
    let last_id = allidstd().len();
 
    let start_key = allids.first().unwrap().clone();

    let end_key = format!("student_{}",last_id +1);

   
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
    let student_key = format!("student_{}", id);

    match db_config::get_data(student_key).await {
        Ok(student_data) => {
            let student_vec: Student = match serde_json::from_str(&student_data) {
                Ok(student) => student,
                Err(err) => {
                    let message = Message {
                        status: 500,
                        message_key: err.to_string(),
                        data: String::from("Failed to deserialize student data"),
                    };
                    return Json(message).into_response();
                }
            };

            let message = Message {
                status: 200,
                message_key: String::from("success"),
                data: student_vec,
            };
            Json(message).into_response()
        }
        Err(_) => {
            let message = Message {
                status: 404,
                message_key: String::from("error"),
                data: "No data found for the given student ID".to_string(),
            };
            Json(message).into_response()
        }
    }
}


pub async fn delete_student_id(Path(id): Path<i32>) -> Response {
  
    let deletion_result = db_config::delete_data(format!("student_{}", id)).await;

   
    if deletion_result {
      
        let message = Message {
            status: 200,
            message_key: String::from("success"),
            data: "Student data deleted successfully".to_string(),
        };
        Json(message).into_response()
    } else {
        
        let message = Message {
            status: 500, 
            message_key: String::from("error"),
            data: "Failed to delete student data".to_string(),
        };
        Json(message).into_response()
    }
}


pub async fn delete_all_student() -> Response {
    let allids = allidstd();
    let last_id = allidstd().len();
 
    let start_key = allids.first().unwrap().clone();
    let end_key = format!("student_{}",last_id +1);

    
    let deletion_result = db_config::delete_data_range(start_key, end_key).await;

    if deletion_result {
        let message = Message {
            status: 200,
            message_key: String::from("success"),
            data: "Student data deleted successfully".to_string(),
        };
        Json(message).into_response()
    } else {
        let message = Message {
            status: 500,
            message_key: String::from("error"),
            data: "Failed to delete student data".to_string(),
        };
        Json(message).into_response()
    }
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



pub async fn create_student(Json(new_students): Json<Vec<Student>>) -> Response {
    let mut response_data: Vec<String> = Vec::new();

    
    let mut all_ids = allidstd();

    for new_student in new_students.iter() {
     
        let new_id = format!("student_{}", all_ids.len() + 1);
     
        all_ids.push(new_id.clone());

        
        let got_student_data = serde_json::to_string(&new_student).expect("Failed to serialize new student data");

        
        match db_config::put_data(new_id.clone(), got_student_data).await {
            Ok(_) => {
                
                response_data.push(format!("Successfully created student with ID: {}", new_id));
            }
            Err(_) => {
              
                response_data.push(format!("Failed to create student with ID: {}", new_id));
            }
        }
    }

 
    let status_code = if response_data.iter().all(|msg| msg.contains("Successfully")) {
        200
    } else {
        400
    };

    let message = Message {
        status: status_code,
        message_key: String::from("success"),
        data: response_data,
    };

    Json(message).into_response()
}
