use axum::{
    extract::Path, response::{IntoResponse, Response}, Json
};

use crate::Axum_server::Users::{Message, master::model::{ALL_MASTER,MasterData}};

// create master data.

// in json body write like this

// {
//     "id": 10,
//     "name": "Rajiv Sharma",
//     "skills": [
//       "Proactive Engagement",
//       "Technical Proficiency",
//       "Cultural Sensitivity"
//     ],
//     "status": "Offline",
//     "language": "Spanish"
//   }


pub async fn create_master(Json(new_master): Json<MasterData>) -> Response {
    let mut master = ALL_MASTER.write().unwrap();
    
    // Check if the ID already exists
    if master.iter().any(|master| master.id == new_master.id) {
        return Json(Message {
            status: 4000,
            message_key: String::from("Error: master data with this ID already exists"),
            data: String::from("Error: no data will be entered"), // Empty data since the student wasn't created
        }).into_response();
    }

    // If ID doesn't exist, push the new student
    master.push(new_master);

    Json(Message {
        status: 2000,
        message_key: String::from("New master data created successfully"),
        data: master.clone(),
    }).into_response()
}





// update master data through json body.
// for e.g. write this in json body.
// {
//     "id": 10,
//     "name": "Rajiv Sharma",
//     "skills": [
//       "Proactive Engagement",
//       "Technical Proficiency",
//       "Cultural Sensitivity"
//     ],
//     "status": "Offline",
//     "language": "Spanish"
//   }

pub async fn update_master(Json(new_master): Json<MasterData>) -> Response {
    let mut master = ALL_MASTER.write().unwrap();
    if let Some(master_data) = master.iter_mut().find(|master| master.id == new_master.id) {
        master_data.name = new_master.name;
        master_data.skills = new_master.skills;
        master_data.status = new_master.status;
        master_data.language = new_master.language;
        

        Json(Message {
            status: 2000,
            message_key: String::from("updated the master data successfully"),
            data: master.clone(),
        })
        .into_response()
    } else {
        master.push(new_master.clone());
        Json(Message {
            status: 4000,
            message_key: String::from("no Such Data Found with that id, created new master with this id and its data"),
            data: new_master.clone(),
        })
        .into_response()
    }
}

// delete employee by its id.

pub async fn delete_master_by_id(Path(id): Path<u32>) -> Response {
    let mut master_id = ALL_MASTER.write().unwrap();

    if let Some(master_id_data) = master_id
        .iter()
        .position(|master_data| master_data.id == id)
    {
        master_id.remove(master_id_data);
        Json(Message {
            status: 2000,
            message_key: String::from("deleted the data successfully"),
            data: master_id.clone(),
        })
        .into_response()
    } else {
        Json(Message {
            status: 4000,
            message_key: String::from("no Such Data Found with that id"),
            data: String::from("no data found with this master id"),
        })
        .into_response()
    }
}



pub async fn get_master_by_id(Path(id): Path<u32>) -> Response {
    let  master_id = ALL_MASTER.write().unwrap();
    let master_id_data = master_id.iter().find(|master_data| master_data.id == id);

    Json(Message {
        status: 2000,
        message_key: String::from("success"),
        data: master_id_data.clone(),
    })
    .into_response()
}

pub async fn get_all_master() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: ALL_MASTER.read().unwrap().clone(),
        })
    }
    .into_response();
}
