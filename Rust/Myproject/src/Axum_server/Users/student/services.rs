
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};

use crate::Axum_server::Users::{ALL_STUDENT, Message,AStudent};

// create student data.

// in json body write like this

// {
//     "id":99,
//       "name": "dhvani bhesaniya",
//       "phone": "+91 123456789",
//       "email": "dhvani@bhesaniya.com",
//       "city": "gondal",
//       "address": "india, gujarat",
//       "marks": [56, 99, 99, 92, 88]
//   }


pub async fn create_student(Json(new_student): Json<AStudent>) -> Response {
    let mut students = ALL_STUDENT.write().unwrap();
    
    // Check if the ID already exists
    if students.iter().any(|student| student.id == new_student.id) {
        return Json(Message {
            status: 4000,
            message_key: String::from("Error: Student with this ID already exists"),
            data: String::from("Error: no data will be entered"), // Empty data since the student wasn't created
        }).into_response();
    }

    // If ID doesn't exist, push the new student
    students.push(new_student);

    Json(Message {
        status: 2000,
        message_key: String::from("New student data created successfully"),
        data: students.clone(),
    }).into_response()
}





// update student data by its id.

// put all content in url like ( http://localhost:3000/student_update/3/aman kailash/123456456/aman@kailash/surat/vadodara) 
// and for marks go in json body and write  the marks you want to update [1,2,3,4,5]

pub async fn update_student_by_id(
    Path((id, name, email, phone, city, address)): Path<(
        u32,
        String,
        String,
        String,
        String,
        String,
        
    )>,
    Json(new_marks): Json<Vec<u32>>,
) -> Response {
    let mut students = ALL_STUDENT.write().unwrap();
    if let Some(student_data) = students.iter_mut().find(|student| student.id == id) {
        student_data.name = name;
        student_data.email = email;
        student_data.phone = phone;
        student_data.city = city;
        student_data.address = address;
        student_data.marks = new_marks;

        Json(Message {
            status: 2000,
            message_key: String::from("updated the student data successfully"),
            data: student_data.clone(),
        })
        .into_response()
    } else {
        Json(Message {
            status: 4000,
            message_key: String::from("no Such Data Found with that id"),
            data: String::from("no data found with this student id"),
        })
        .into_response()
    }
}

// delete student by its id.

pub async fn delete_student_by_id(Path(id): Path<u32>) -> Response {
    let mut student_id = ALL_STUDENT.write().unwrap();

    if let Some(student_id_data) = student_id
        .iter()
        .position(|student_data| student_data.id == id)
    {
        student_id.remove(student_id_data);
        Json(Message {
            status: 2000,
            message_key: String::from("deleted the data successfully"),
            data: student_id.clone(),
        })
        .into_response()
    } else {
        Json(Message {
            status: 4000,
            message_key: String::from("no Such Data Found with that id"),
            data: String::from("no data found with this student id"),
        })
        .into_response()
    }
}

// get student data by its id.

pub async fn get_student_by_id(Path(id): Path<u32>) -> Response {
    let student_id = ALL_STUDENT.write().unwrap();
    let student_id_data = student_id.iter().find(|student_data| student_data.id == id);

    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: student_id_data.clone(),
    })
    .into_response()
}

// real all student data

pub async fn get_all_students() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: ALL_STUDENT.read().unwrap().clone(),
        })
    }
    .into_response();
}
