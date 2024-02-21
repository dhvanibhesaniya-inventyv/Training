// use tikv_client::{RawClient, TransactionClient};


// pub async fn get_client() -> RawClient {
//     let client = RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap();
//     client
// }
// pub async fn add_record(key: String, value: String) {
//    let client = get_client().await;
//     client.put(key.to_string(), value.to_string()).await.unwrap();
//     println!("Record Added With Key :{:?}", key);
// }


// pub async fn get_record(key: String) {
//     let client = RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap();


//     let value = client.get(key.to_string()).await.unwrap();
//     println!("Valuee  :{:?}", String::from_utf8(value.unwrap()));
// }
