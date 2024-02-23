use services::master_data_client::MasterDataClient;
use services::{GetMasterRequest,GetMasterIdRequest,DeteleMasterRequest,PutMasterRequest,CreateMasterRequest,UpdateMRealJsonRequest };

pub mod services {
    tonic::include_proto!("services");
}

pub async fn getallMaster(){
    let mut client = MasterDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    
    let request = tonic::Request::new(GetMasterRequest {
        mstdata: "get Master data".to_string(), 
    });
    
    // Call the RPC
    let response = client.get_master_data(request).await.unwrap();
    
    // Process response
    println!("Response: {:?}", response);
    
    
}

pub async fn getMasterbyid(){
    let mut client = MasterDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    let request2 = tonic::Request::new(GetMasterIdRequest{
        id: 1,
    });
    
    let response2 = client.get_master_id(request2).await.unwrap();
      // Process response
      println!("Response: {:#?}", response2);
    
    
}
pub async fn deleteMasterbyid(){
    let mut client = MasterDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();


let request3 = tonic::Request::new(DeteleMasterRequest{
    id: 1,
});

let response3 = client.delete_master(request3).await.unwrap();
  // Process response
  println!("Response: {:#?}", response3);


}
pub async fn updateMasterbyid(){
    
    let mut client = MasterDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();


  // Create a PutMasterRequest with the desired Master data
  let request5 = tonic::Request::new(PutMasterRequest {
    id: 2,
    name: "dhvani".to_string(),
    skill: vec!["abcd".to_string(),"abc".to_string()],
    language: "English".to_string(),
    status: "Online".to_string(),
});

// Send the request to the server
let response5 = client.put_master(request5).await.unwrap();

// Process the response
println!("Response: {:?}", response5);


}



pub async fn createnewMaster(){


    let mut client = MasterDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

     // Create a CreateMasterRequest with the desired Master data
 let request6 = CreateMasterRequest {
    id:Some(99),
    name: "John Doe".to_string(),
    skill: vec!["abcd".to_string(),"abc".to_string()],
    language: "English".to_string(),
    status: "Online".to_string(),
    
};

// Send the request to the server
let response6 = client.create_master(request6).await.unwrap();

// Process the response
println!("Response: {:?}", response6);


}


pub async fn updatemrealjsonfile(){

    let mut client = MasterDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    

// Create a UpdateRealJsonRequest 
let request7 = UpdateMRealJsonRequest {
    updateit: "Yes update the  real Master json file".to_string(),
};

   // Send the request to the server
   let response7 = client.update_m_real_json(request7).await.unwrap();

   // Process the response
   println!("Response: {:?}", response7);


    // Prepare request with stddata field set
    let request4 = tonic::Request::new(GetMasterRequest {
        mstdata: "get Master data".to_string(), 
    });

    // Call the RPC
    let response4 = client.get_master_data(request4).await.unwrap();

    // Process response
    println!("Response: {:#?}", response4);
}







