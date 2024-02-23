use services::employee_data_client::EmployeeDataClient;
use services::{GetEmployeeRequest,GetEmployeeIdRequest,DeteleEmployeeRequest,PutEmployeeRequest,CreateEmployeeRequest,UpdateERealJsonRequest };

pub mod services {
    tonic::include_proto!("services");
}

pub async fn getallemployee(){
    let mut client = EmployeeDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    
    let request = tonic::Request::new(GetEmployeeRequest {
        empdata: "get employee data".to_string(), 
    });
    
    // Call the RPC
    let response = client.get_employee_data(request).await.unwrap();
    
    // Process response
    println!("Response: {:?}", response);
    
    
}

pub async fn getemployeebyid(){
    let mut client = EmployeeDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    let request2 = tonic::Request::new(GetEmployeeIdRequest{
        id: 1,
    });
    
    let response2 = client.get_employee_id(request2).await.unwrap();
      // Process response
      println!("Response: {:#?}", response2);
    
    
}
pub async fn deleteemployeebyid(){
    let mut client = EmployeeDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();


let request3 = tonic::Request::new(DeteleEmployeeRequest{
    id: 1,
});

let response3 = client.delete_employee(request3).await.unwrap();
  // Process response
  println!("Response: {:#?}", response3);


}
pub async fn updateemployeebyid(){
    
    let mut client = EmployeeDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();


  // Create a PutEmployeeRequest with the desired Employee data
  let request5 = tonic::Request::new(PutEmployeeRequest {
    id: 2,
    name: "dhvani".to_string(),
    age: 40,
    skill: vec!["abcd".to_string(),"abc".to_string()],
    position: Some("jr, software developer".to_string()),
    exprience: Some(5),
});

// Send the request to the server
let response5 = client.put_employee(request5).await.unwrap();

// Process the response
println!("Response: {:?}", response5);


}



pub async fn createnewemployee(){


    let mut client = EmployeeDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

     // Create a CreateEmployeeRequest with the desired Employee data
 let request6 = CreateEmployeeRequest {
    id:Some(99),
    name: "John Doe".to_string(),
    age: 56,
    skill: vec!["abcd".to_string(),"abc".to_string()],
    position: Some("jr. software developer".to_string()),
    exprience: Some(5),
    
};

// Send the request to the server
let response6 = client.create_employee(request6).await.unwrap();

// Process the response
println!("Response: {:?}", response6);


}


pub async fn updateerealjsonfile(){

    let mut client = EmployeeDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    

// Create a UpdateRealJsonRequest 
let request7 = UpdateERealJsonRequest {
    updateit: "Yes update the  real employee json file".to_string(),
};

   // Send the request to the server
   let response7 = client.update_e_real_json(request7).await.unwrap();

   // Process the response
   println!("Response: {:?}", response7);


    // Prepare request with stddata field set
    let request4 = tonic::Request::new(GetEmployeeRequest {
        empdata: "get employee data".to_string(), 
    });

    // Call the RPC
    let response4 = client.get_employee_data(request4).await.unwrap();

    // Process response
    println!("Response: {:#?}", response4);
}







