use services::employee_data_server::EmployeeData;
use services::{
    CreateEmployeeRequest, CreateEmployeeResponse, DeteleEmployeeRequest, DeteleEmployeeResponse,
    GetEmployeeIdRequest, GetEmployeeIdResponse, GetEmployeeRequest, GetEmployeeResponse,
    PutEmployeeRequest, PutEmployeeResponse, UpdateERealJsonRequest, UpdateERealJsonResponse,
};
use std::fs;



use tonic::{ Request, Response, Status};

use crate::Users::employee::model::{ AEmployee, ALL_EMPLOYEE };


pub mod services {
    tonic::include_proto!("services");
}

#[derive(Debug, Default)]
pub struct EmployeeServices {}

#[tonic::async_trait]
impl EmployeeData for EmployeeServices {


    // getting all data from student json data
    
    async fn get_employee_data(
        &self,
        _request: Request<GetEmployeeRequest>,
    ) -> Result<Response<GetEmployeeResponse>, Status> {
        // Acquire read lock on ALLEmployee
        let all_employees = ALL_EMPLOYEE.read().unwrap();

        // Serialize all students to JSON
        let all_employee_json = serde_json::to_string(&*all_employees).unwrap();

        let reply = GetEmployeeResponse {
            status: 200,
            message_key: "success".to_string(),
            data: all_employee_json,
        };

        Ok(Response::new(reply))
    }



    // getting specific data from student json data using id.

    async fn get_employee_id(
        &self,
        request: Request<GetEmployeeIdRequest>,
    ) -> Result<Response<GetEmployeeIdResponse>, Status> {

        // Acquire read lock on ALL_STUDENT
        let all_employee = ALL_EMPLOYEE.read().unwrap();
        let request_data = request.into_inner().id;

        // Find student with requested ID
        let employee_id_data = all_employee
            .iter()
            .find(|employee_data| employee_data.id == request_data);

        // Checking if the student with requested ID is found.
        if let Some(employee) = employee_id_data {

            // Serialize found student data to JSON
            let employee_json = serde_json::to_string(employee).unwrap();

            let reply = GetEmployeeIdResponse {
                status: 200,
                message_key: "success".to_string(),
                data: employee_json,
            };

            Ok(Response::new(reply))
        } else {
            // If student with requested ID is not found, return an error response
            let reply = GetEmployeeIdResponse {
                status: 404, 
                message_key: "error".to_string(),
                data: "Student not found".to_string(),
            };

            Ok(Response::new(reply))
        }
    }



    // deleting specific data from student json data using id

    async fn delete_employee(
        &self,
        request: Request<DeteleEmployeeRequest>,
    ) -> Result<Response<DeteleEmployeeResponse>, Status> {
        // Acquire write lock on ALL_EMPLOYEE
        let mut all_employees = ALL_EMPLOYEE.write().unwrap();
        let id_to_delete = request.into_inner().id;

        // Remove the student with the specified ID.
        if let Some(index) = all_employees
            .iter()
            .position(|employee_data| employee_data.id == id_to_delete)
        {
            all_employees.remove(index);
            let reply = DeteleEmployeeResponse {
                status: 200, 
                message_key: "success".to_string(),
                data: "Empoyee deleted successfully".to_string(),
            };

            Ok(Response::new(reply))
        } else {
            // If student with the specified ID is not found, return an error response.
            let reply = DeteleEmployeeResponse {
                status: 404, 
                message_key: "error".to_string(),
                data: "Employee not found".to_string(),
            };
            Ok(Response::new(reply))
        }
    }


    // updating specific data to student json data
    async fn put_employee(
        &self,
        request: Request<PutEmployeeRequest>,
    ) -> Result<Response<PutEmployeeResponse>, Status> {
        let PutEmployeeRequest {
            id,
            name,
            age,
            skill,
            position,
            exprience,

        } = request.into_inner();


        // Acquire write lock on ALL_STUDENT to modify student data
        let mut all_employee = ALL_EMPLOYEE.write().unwrap();

        // Find the student with the provided ID
        if let Some(employee) = all_employee.iter_mut().find(|employee| employee.id == id) {
             // Update employee information
        
            employee.name = name;
      
            employee.age = age;
        
            employee.skills = skill;
        
        if let Some(position) = position {
            employee.position = Some(position);
        }
        if let Some(experience) = exprience {
            employee.experience = Some(experience);
        }

        // Construct the response indicating that the employee has been updated
        let response = PutEmployeeResponse {
            status: 200, // Success status code
            message_key: "success".to_string(),
            data: format!("Employee with ID {} has been updated", id),
        };
        Ok(Response::new(response))
    } else {
        // If employee with the provided ID is not found, return an error response
        let response = PutEmployeeResponse {
            status: 404, // Not Found status code
            message_key: "error".to_string(),
            data: "Employee not found".to_string(),
        };
        Ok(Response::new(response))
    }
    }

    // // create employee data.

    // // in json body write like this

    // // {
    // //     "id":99,
    // //       "name": "dhvani bhesaniya",
    // //       "skills": ["abc","abc"],
    // //       "position": "software developer",
    // //       "experience": 5
    // //       
    // //   }

    async fn create_employee(
        &self,
        request: Request<CreateEmployeeRequest>,
    ) -> Result<Response<CreateEmployeeResponse>, Status> {
        // Acquire write lock on ALL_EMPLOYEE
        let mut all_employee = ALL_EMPLOYEE.write().unwrap();
    
        // Extract CreateEmployeeRequest from the incoming request
        let employee_request = request.into_inner();
    
        // Generate a new id by incrementing the last id by 1
        let new_id = if let Some(last_employee) = all_employee.last() {
            last_employee.id + 1
        } else {
            // if the id is not found then start with 1.
            1 
        };
    
        // Construct a new employee object with the generated id and request data
        let mut new_employee = AEmployee {
            id: new_id, // or employee_request.id
            name: employee_request.name,
            age: employee_request.age,
            skills: employee_request.skill,
            position: None, // Default to None
            experience: None, // Default to None
        };
    
        // If experience is provided in the request, set it in the new employee object
        if let Some(experience) = employee_request.exprience {
            new_employee.experience = Some(experience);
        }
    
        // If position is provided in the request, set it in the new employee object
        if let Some(position) = employee_request.position {
            new_employee.position = Some(position);
        }
    
        // Add the new employee to the ALL_EMPLOYEE vector
        all_employee.push(new_employee);
    
        // Prepare the response
        let response = CreateEmployeeResponse {
            status: 200,
            message_key: "success".to_string(),
            data: format!("New employee with id {} has been created", new_id),
        };
    
        Ok(Response::new(response))
    }
    

    // this function is to update the real json data  from the changes made in the lazystatic All_STUDENT vector.
    async fn update_e_real_json(
        &self,
        request: Request<UpdateERealJsonRequest>,
    ) -> Result<Response<UpdateERealJsonResponse>, Status> {
       
        let _json_update_request = request.into_inner();

       
        let all_employee = ALL_EMPLOYEE.write().unwrap();

        // Serialize the updated employee data to JSON
        let updated_data =
            serde_json::to_string_pretty(&*all_employee).expect("Failed to serialize JSON");

        // Write the updated data back to the file
        let file_path = "utils/Axum_server_json/Employee.json";
        if let Err(err) = fs::write(file_path, updated_data) {
            return Err(Status::internal(format!(
                "Failed to update data in file {}: {}",
                file_path, err
            )));
        }

        
        let response = UpdateERealJsonResponse {
            status: 200,
            message_key: "success".to_string(),
            data: format!("Employee Json file has been updated"),
        };

     
        Ok(Response::new(response))
    }
}
