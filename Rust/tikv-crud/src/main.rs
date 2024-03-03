

use users::student::model::Student;
use tikv_client::RawClient;

use users::student::model::parse_json_data;


pub mod users;
pub mod utils;


pub async fn store_data_in_tikv(students: Vec<Student>, client: &RawClient) {
    let mut counter = 1; 

    for student in students {
        let key = format!("student_{}", counter);
        let value = serde_json::to_string(&student).expect("Failed to serialize student");
        client.put(key, value).await.expect("Failed to store data in TiKV");

        counter += 1; 
    }
}


#[tokio::main]
async fn main() {
    let client  = utils::db_config::get_client().await;
    //let client = RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap();
    

    let students = parse_json_data("./json_files/StudentData.json".to_string());
    store_data_in_tikv(students, &client).await;

    println!("running on http://127.0.0.1:5000");
    let app = users::student::api();
    axum::Server::bind(&"127.0.0.1:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
