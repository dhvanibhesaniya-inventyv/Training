use services::student_data_server::StudentData;
use services::{
    CreateStudentRequest, CreateStudentResponse, DeteleStudentRequest, DeteleStudentResponse,
    GetStudentIdRequest, GetStudentIdResponse, GetStudentRequest, GetStudentResponse,
    PutStudentRequest, PutStudentResponse, UpdateRealJsonRequest, UpdateRealJsonResponse,
};
use std::fs;
use std::str::FromStr;
use tonic::{ Request, Response, Status};

use crate::Users::student::model::{AStudent, ALL_STUDENT};

pub mod services {
    tonic::include_proto!("services");
}

#[derive(Debug, Default)]
pub struct StudentServices {}

#[tonic::async_trait]
impl StudentData for StudentServices {


    // getting all data from student json data
    
    async fn get_student_data(
        &self,
        _request: Request<GetStudentRequest>,
    ) -> Result<Response<GetStudentResponse>, Status> {
        // Acquire read lock on ALL_STUDENT
        let all_students = ALL_STUDENT.read().unwrap();

        // Serialize all students to JSON
        let all_students_json = serde_json::to_string(&*all_students).unwrap();

        let reply = GetStudentResponse {
            status: 200,
            message_key: "success".to_string(),
            data: all_students_json,
        };

        Ok(Response::new(reply))
    }



    // getting specific data from student json data using id.

    async fn get_student_id(
        &self,
        request: Request<GetStudentIdRequest>,
    ) -> Result<Response<GetStudentIdResponse>, Status> {

        // Acquire read lock on ALL_STUDENT
        let all_students = ALL_STUDENT.read().unwrap();
        let request_data = request.into_inner().id;

        // Find student with requested ID
        let student_id_data = all_students
            .iter()
            .find(|student_data| student_data.id == request_data);

        // Checking if the student with requested ID is found.
        if let Some(student) = student_id_data {

            // Serialize found student data to JSON
            let student_json = serde_json::to_string(student).unwrap();

            let reply = GetStudentIdResponse {
                status: 200,
                message_key: "success".to_string(),
                data: student_json,
            };

            Ok(Response::new(reply))
        } else {
            // If student with requested ID is not found, return an error response
            let reply = GetStudentIdResponse {
                status: 404, 
                message_key: "error".to_string(),
                data: "Student not found".to_string(),
            };

            Ok(Response::new(reply))
        }
    }



    // deleting specific data from student json data using id

    async fn delete_student(
        &self,
        request: Request<DeteleStudentRequest>,
    ) -> Result<Response<DeteleStudentResponse>, Status> {
        // Acquire write lock on ALL_STUDENT
        let mut all_students = ALL_STUDENT.write().unwrap();
        let id_to_delete = request.into_inner().id;

        // Remove the student with the specified ID.
        if let Some(index) = all_students
            .iter()
            .position(|student_data| student_data.id == id_to_delete)
        {
            all_students.remove(index);
            let reply = DeteleStudentResponse {
                status: 200, 
                message_key: "success".to_string(),
                data: "Student deleted successfully".to_string(),
            };

            Ok(Response::new(reply))
        } else {
            // If student with the specified ID is not found, return an error response.
            let reply = DeteleStudentResponse {
                status: 404, 
                message_key: "error".to_string(),
                data: "Student not found".to_string(),
            };
            Ok(Response::new(reply))
        }
    }


    // updating specific data to student json data
    async fn put_student(
        &self,
        request: Request<PutStudentRequest>,
    ) -> Result<Response<PutStudentResponse>, Status> {
        let PutStudentRequest {
            id,
            name,
            phone,
            email,
            city,
            address,
            marks,
        } = request.into_inner();
        // Convert marks from Vec<String> to Vec<u32>
        let marks: Result<Vec<u32>, _> = marks.iter().map(|s| u32::from_str(s)).collect();
        let marks = match marks {
            Ok(marks) => marks,
            Err(_) => return Err(Status::invalid_argument("Invalid marks format")),
        };

        // Acquire write lock on ALL_STUDENT to modify student data
        let mut all_students = ALL_STUDENT.write().unwrap();

        // Find the student with the provided ID
        if let Some(student) = all_students.iter_mut().find(|student| student.id == id) {
            // Update student information
            student.name = name;
            student.phone = phone;
            student.email = email;
            student.city = city;
            student.address = address;
            student.marks = marks;

            // Construct the response indicating that the student has been updated
            let response = PutStudentResponse {
                status: 200, // Success status code
                message_key: "success".to_string(),
                data: format!("Student with ID {} has been updated", id),
            };
            Ok(Response::new(response))
        } else {
            // If student with the provided ID is not found, return an error response
            let response = PutStudentResponse {
                status: 404, // Not Found status code
                message_key: "error".to_string(),
                data: "Student not found".to_string(),
            };
            Ok(Response::new(response))
        }
    }

    // // create student data.

    // // in json body write like this

    // // {
    // //     "id":99,
    // //       "name": "dhvani bhesaniya",
    // //       "phone": "+91 123456789",
    // //       "email": "dhvani@bhesaniya.com",
    // //       "city": "gondal",
    // //       "address": "india, gujarat",
    // //       "marks": [56, 99, 99, 92, 88]
    // //   }

    async fn create_student(
        &self,
        request: Request<CreateStudentRequest>,
    ) -> Result<Response<CreateStudentResponse>, Status> {
        // Acquire write lock on ALL_STUDENT
        let mut all_students = ALL_STUDENT.write().unwrap();

        // Extract CreateStudentRequest from the incoming request
        let student_request = request.into_inner();

        // Convert marks from Vec<String> to Vec<u32>
        let marks: Result<Vec<u32>, _> = student_request
            .marks
            .iter()
            .map(|s| u32::from_str(s))
            .collect();
        let marks = match marks {
            Ok(marks) => marks,
            Err(_) => return Err(Status::invalid_argument("Invalid marks format")),
        };

        // Generate a new id by incrementing the last id by 1
        let new_id = if let Some(last_student) = all_students.last() {
            last_student.id + 1
        } else {
            // if the is is not found then start with 1.
            1 
        };

        // Construct a new student object with the generated id and request data, (here id can be genereted new or can take id from client side also)
        let new_student = AStudent {
            id: new_id, // or student_request.id
            name: student_request.name,
            phone: student_request.phone,
            email: student_request.email,
            city: student_request.city,
            address: student_request.address,
            marks: marks,
        };

        // Add the new student to the ALL_STUDENT vector
        all_students.push(new_student);

        // Prepare the response
        let response = CreateStudentResponse {
            status: 200,
            message_key: "success".to_string(),
            data: format!("New student with id {} has been created", new_id),
        };

        Ok(Response::new(response))
    }

    // this function is to update the real json data  from the changes made in the lazystatic All_STUDENT vector.
    async fn update_real_json(
        &self,
        request: Request<UpdateRealJsonRequest>,
    ) -> Result<Response<UpdateRealJsonResponse>, Status> {
       
        let _json_update_request = request.into_inner();

       
        let all_students = ALL_STUDENT.write().unwrap();

        // Serialize the updated student data to JSON
        let updated_data =
            serde_json::to_string_pretty(&*all_students).expect("Failed to serialize JSON");

        // Write the updated data back to the file
        let file_path = "utils/Axum_server_json/StudentData.json";
        if let Err(err) = fs::write(file_path, updated_data) {
            return Err(Status::internal(format!(
                "Failed to update data in file {}: {}",
                file_path, err
            )));
        }

        
        let response = UpdateRealJsonResponse {
            status: 200,
            message_key: "success".to_string(),
            data: format!("StudentJson data has been updated"),
        };

     
        Ok(Response::new(response))
    }
}
