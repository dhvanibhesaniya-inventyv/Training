use services::master_data_server::MasterData;
use services::{
    CreateMasterRequest, CreateMasterResponse, DeteleMasterRequest, DeteleMasterResponse,
    GetMasterRequest, GetMasterResponse, PutMasterRequest, PutMasterResponse,
    UpdateMRealJsonRequest, UpdateMRealJsonResponse, GetMasterIdRequest, GetMasterIdResponse,
};
use std::fs;



use tonic::{ Request, Response, Status};

use crate::Users::master::model::{MasterData as StructMasterData , ALL_MASTER };


pub mod services {
    tonic::include_proto!("services");
}

#[derive(Debug, Default)]
pub struct MasterServices {}

#[tonic::async_trait]
impl MasterData for MasterServices {


    // getting all data from Master json data
    
    async fn get_master_data(
        &self,
        _request: Request<GetMasterRequest>,
    ) -> Result<Response<GetMasterResponse>, Status> {
        // Acquire read lock on ALLMaster
        let all_masters = ALL_MASTER.read().unwrap();

        // Serialize all MASTER to JSON
        let all_master_json = serde_json::to_string(&*all_masters).unwrap();

        let reply = GetMasterResponse {
            status: 200,
            message_key: "success".to_string(),
            data: all_master_json,
        };

        Ok(Response::new(reply))
    }



    // getting specific data from Master json data using id.

    async fn get_master_id(
        &self,
        request: Request<GetMasterIdRequest>,
    ) -> Result<Response<GetMasterIdResponse>, Status> {

        // Acquire read lock on ALL_MASTER
        let all_master = ALL_MASTER.read().unwrap();
        let request_data = request.into_inner().id;

        // Find Master with requested ID
        let master_id_data = all_master
            .iter()
            .find(|master_data| master_data.id == request_data);

        // Checking if the Master with requested ID is found.
        if let Some(master) = master_id_data {

            // Serialize found Master data to JSON
            let master_json = serde_json::to_string(master).unwrap();

            let reply = GetMasterIdResponse {
                status: 200,
                message_key: "success".to_string(),
                data: master_json,
            };

            Ok(Response::new(reply))
        } else {
            // If Master with requested ID is not found, return an error response
            let reply = GetMasterIdResponse {
                status: 404, 
                message_key: "error".to_string(),
                data: "Master not found".to_string(),
            };

            Ok(Response::new(reply))
        }
    }



    // deleting specific data from Master json data using id

    async fn delete_master(
        &self,
        request: Request<DeteleMasterRequest>,
    ) -> Result<Response<DeteleMasterResponse>, Status> {
        // Acquire write lock on ALL_MASTER
        let mut all_masters = ALL_MASTER.write().unwrap();
        let id_to_delete = request.into_inner().id;

        // Remove the Master with the specified ID.
        if let Some(index) = all_masters
            .iter()
            .position(|master_data| master_data.id == id_to_delete)
        {
            all_masters.remove(index);
            let reply = DeteleMasterResponse {
                status: 200, 
                message_key: "success".to_string(),
                data: "Master deleted successfully".to_string(),
            };

            Ok(Response::new(reply))
        } else {
            // If Master with the specified ID is not found, return an error response.
            let reply = DeteleMasterResponse {
                status: 404, 
                message_key: "error".to_string(),
                data: "Master not found".to_string(),
            };
            Ok(Response::new(reply))
        }
    }


    // updating specific data to Master json data
    async fn put_master(
        &self,
        request: Request<PutMasterRequest>,
    ) -> Result<Response<PutMasterResponse>, Status> {
        let PutMasterRequest {
            id,
            name,
            skill,
            language,
            status,

        } = request.into_inner();


        // Acquire write lock on ALL_Master to modify Master data
        let mut all_master = ALL_MASTER.write().unwrap();

        // Find the Master with the provided ID
        if let Some(master) = all_master.iter_mut().find(|master| master.id == id) {
             // Update Master information
        
            master.name = name;
            master.skills = skill;
            master.language = language;
            master.status = status;
        
        

        // Construct the response indicating that the Master has been updated
        let response = PutMasterResponse {
            status: 200, // Success status code
            message_key: "success".to_string(),
            data: format!("Master with ID {} has been updated", id),
        };
        Ok(Response::new(response))
    } else {
        // If Master with the provided ID is not found, return an error response
        let response = PutMasterResponse {
            status: 404, // Not Found status code
            message_key: "error".to_string(),
            data: "Master not found".to_string(),
        };
        Ok(Response::new(response))
    }
    }

    // // create Master data.

    // // in json body write like this

    // // {
    // //     "id":99,
    // //       "name": "dhvani bhesaniya",
    // //       "skills": ["abc","abc"],
    // //       "language": "English",
    // //       "status": "online"
    // //       
    // //   }

    async fn create_master(
        &self,
        request: Request<CreateMasterRequest>,
    ) -> Result<Response<CreateMasterResponse>, Status> {
        // Acquire write lock on ALL_Master
        let mut all_master = ALL_MASTER.write().unwrap();
    
        // Extract CreateMasterRequest from the incoming request
        let master_request = request.into_inner();
    
        // Generate a new id by incrementing the last id by 1
        let new_id = if let Some(last_master) = all_master.last() {
            last_master.id + 1
        } else {
            // if the id is not found then start with 1.
            1 
        };
    
        // Construct a new Master object with the generated id and request data
        let  new_master = StructMasterData {
            id: new_id, // or Master_request.id
            name: master_request.name,
            skills: master_request.skill,
            language: master_request.language,
            status: master_request.status,
        };
    
        // Add the new Master to the ALL_MASTER vector
        all_master.push(new_master);
    
        // Prepare the response
        let response = CreateMasterResponse {
            status: 200,
            message_key: "success".to_string(),
            data: format!("New Master with id {} has been created", new_id),
        };
    
        Ok(Response::new(response))
    }
    

    // this function is to update the real json data  from the changes made in the lazystatic All_Master vector.
    async fn update_m_real_json(
        &self,
        request: Request<UpdateMRealJsonRequest>,
    ) -> Result<Response<UpdateMRealJsonResponse>, Status> {
       
        let _json_update_request = request.into_inner();

       
        let all_master = ALL_MASTER.write().unwrap();

        // Serialize the updated Master data to JSON
        let updated_data =
            serde_json::to_string_pretty(&*all_master).expect("Failed to serialize JSON");

        // Write the updated data back to the file
        let file_path = "utils/Axum_server_json/Master_Data.json";
        if let Err(err) = fs::write(file_path, updated_data) {
            return Err(Status::internal(format!(
                "Failed to update data in file {}: {}",
                file_path, err
            )));
        }

        
        let response = UpdateMRealJsonResponse {
            status: 200,
            message_key: "success".to_string(),
            data: format!("Master Json file has been updated"),
        };

     
        Ok(Response::new(response))
    }
}
