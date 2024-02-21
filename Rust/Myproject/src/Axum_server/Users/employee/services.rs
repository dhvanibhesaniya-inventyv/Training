use axum::{
    extract::Path, response::{IntoResponse, Response}, Json
};

use crate::Axum_server::Users::{Message, employee::model::{ALL_EMPLOYEE,AEmployee}};


// create employee data.

// in json body write like this

// {
//     "id": 20,
//     "name": "Aryan",
//     "age": 28,
//     "skills": [
//       "Rust",
//       "Python"
//     ],
//     "position": "Sr. Software Developer",
//     "experience(y)": 6
//   }


pub async fn create_employee(Json(new_employee): Json<AEmployee>) -> Response {
    let mut employee = ALL_EMPLOYEE.write().unwrap();
    
    // Check if the ID already exists
    if employee.iter().any(|employee| employee.id == new_employee.id) {
        return Json(Message {
            status: 4000,
            message_key: String::from("Error: employee with this ID already exists"),
            data: String::from("Error: no data will be entered"), // Empty data since the student wasn't created
        }).into_response();
    }

    // If ID doesn't exist, push the new student
    employee.push(new_employee);

    Json(Message {
        status: 2000,
        message_key: String::from("New employee data created successfully"),
        data: employee.clone(),
    }).into_response()
}



// update employee data through json body.
// for e.g. write this in json body.
// {
//     "id": 20,
//     "name": "Aryan",
//     "age": 28,
//     "skills": [
//       "Rust",
//       "Python"
//     ],
//     "position": "Sr. Software Developer",
//     "experience(y)": 6
//   }

pub async fn update_employee(Json(new_employee): Json<AEmployee>) -> Response {
    let mut employee = ALL_EMPLOYEE.write().unwrap();
    if let Some(employee_data) = employee.iter_mut().find(|employee| employee.id == new_employee.id) {
        employee_data.name = new_employee.name;
        employee_data.age = new_employee.age;
        employee_data.skills = new_employee.skills;
        employee_data.position = new_employee.position;
        employee_data.experience = new_employee.experience;
        

        Json(Message {
            status: 2000,
            message_key: String::from("updated the employee data successfully"),
            data: employee.clone(),
        })
        .into_response()
    } else {
        employee.push(new_employee.clone());
        Json(Message {
            status: 4000,
            message_key: String::from("no Such Data Found with that id, created new employee with this id and its data"),
            data: new_employee.clone(),
        })
        .into_response()
    }
}

// delete employee by its id.

pub async fn delete_employee_by_id(Path(id): Path<u32>) -> Response {
    let mut employee_id = ALL_EMPLOYEE.write().unwrap();

    if let Some(employee_id_data) = employee_id
        .iter()
        .position(|student_data| student_data.id == id)
    {
        employee_id.remove(employee_id_data);
        Json(Message {
            status: 2000,
            message_key: String::from("deleted the data successfully"),
            data: employee_id.clone(),
        })
        .into_response()
    } else {
        Json(Message {
            status: 4000,
            message_key: String::from("no Such Data Found with that id"),
            data: String::from("no data found with this employee id"),
        })
        .into_response()
    }
}


pub async fn get_employee_by_id(Path(id): Path<u32>) -> Response {
    let  employee_id = ALL_EMPLOYEE.write().unwrap();
    let employee_id_data = employee_id.iter().find(|employee_data| employee_data.id == id);
    if Some(employee_id_data).is_some(){
    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: employee_id_data.clone(),
    })
    .into_response()
}else{
    Json(Message {
        status: 2000,
        message_key: String::from("no data found with this id"),
        data: employee_id_data.clone(),
    })
    .into_response()

}
}

pub async fn get_all_employee() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: ALL_EMPLOYEE.read().unwrap().clone(),
        })
    }
    .into_response();
}
