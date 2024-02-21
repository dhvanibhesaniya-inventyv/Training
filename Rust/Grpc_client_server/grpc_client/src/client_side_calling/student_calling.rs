use services::student_data_client::StudentDataClient;
use services::{GetStudentRequest,GetStudentIdRequest,DeteleStudentRequest,PutStudentRequest,CreateStudentRequest,UpdateRealJsonRequest };

pub mod services {
    tonic::include_proto!("services");
}

pub async fn getallstudent(){
    let mut client = StudentDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    // Prepare request with stddata field set
    let request = tonic::Request::new(GetStudentRequest {
        stddata: "get student data".to_string(), // Set the value of the stddata field
    });
    
    // Call the RPC
    let response = client.get_student_data(request).await.unwrap();
    
    // Process response
    println!("Response: {:?}", response);
    
    
}

pub async fn getstudentbyid(){
    let mut client = StudentDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    let request2 = tonic::Request::new(GetStudentIdRequest{
        id: 1,
    });
    
    let response2 = client.get_student_id(request2).await.unwrap();
      // Process response
      println!("Response: {:#?}", response2);
    
    
}
pub async fn deletestudentbyid(){
    let mut client = StudentDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();


let request3 = tonic::Request::new(DeteleStudentRequest{
    id: 1,
});

let response3 = client.delete_student(request3).await.unwrap();
  // Process response
  println!("Response: {:#?}", response3);


}
pub async fn updatestudentbyid(){
    
    let mut client = StudentDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();


  // Create a PutStudentRequest with the desired student data
  let request5 = tonic::Request::new(PutStudentRequest {
    id: 2,
    name: "dhvani".to_string(),
    phone: "+91 9876543210".to_string(),
    email: "dhvani.patel@example.com".to_string(),
    city: "gondal".to_string(),
    address: "RAJKOT".to_string(),
    marks: vec![1, 1, 1, 1, 1].into_iter().map(|n| n.to_string()).collect(), // Convert marks to strings
});

// Send the request to the server
let response5 = client.put_student(request5).await.unwrap();

// Process the response
println!("Response: {:?}", response5);


}



pub async fn createnewstudent(){


    let mut client = StudentDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

     // Create a CreateStudentRequest with the desired student data
 let request6 = CreateStudentRequest {
    id:Some(99),
    name: "John Doe".to_string(),
    phone: "+91 9876543210".to_string(),
    email: "john.doe@example.com".to_string(),
    city: "New York".to_string(),
    address: "123 Main St".to_string(),
    marks: vec!["80".to_string(), "75".to_string(), "90".to_string(),"90".to_string(), "90".to_string()], // Example marks
};

// Send the request to the server
let response6 = client.create_student(request6).await.unwrap();

// Process the response
println!("Response: {:?}", response6);


}


pub async fn updaterealjsonfile(){

    let mut client = StudentDataClient::connect("grpc://127.0.0.1:50051").await.unwrap();

    

// Create a UpdateRealJsonRequest 
let request7 = UpdateRealJsonRequest {
    updateit: "Yes update the  real student json file".to_string(),
};

   // Send the request to the server
   let response7 = client.update_real_json(request7).await.unwrap();

   // Process the response
   println!("Response: {:?}", response7);


    // Prepare request with stddata field set
    let request4 = tonic::Request::new(GetStudentRequest {
        stddata: "get student data".to_string(), // Set the value of the stddata field
    });

    // Call the RPC
    let response4 = client.get_student_data(request4).await.unwrap();

    // Process response
    println!("Response: {:#?}", response4);
}







