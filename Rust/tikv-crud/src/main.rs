

use users::student::model::Student;
use tikv_client::RawClient;

use users::student::model::parse_json_data;


pub mod users;
pub mod utils;







async fn store_data_in_tikv(students: Vec<Student>, client: &RawClient) {
    for student in students {
        let key = format!("student_{}", student.s_id);
        let value = serde_json::to_string(&student).expect("Failed to serialize student");
        client.put(key ,  value).await.expect("Failed to store data in TiKV");
    }
}


#[tokio::main]
async fn main() {
    let client  = utils::db_config::get_client().await;
    //let client = RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap();
    

    let students = parse_json_data("./json_file/StudentData.json".to_string());
    store_data_in_tikv(students, &client).await;

    let app = users::student::api();
    axum::Server::bind(&"127.0.0.1:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
