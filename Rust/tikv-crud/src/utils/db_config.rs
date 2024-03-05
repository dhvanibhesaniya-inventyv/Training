
// use std::thread;

use tikv_client::{Key, RawClient,  Value};



pub async fn get_client()->RawClient{
  
  RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap()

}
 
pub async fn scan_data(start_key: String, end_key: String) -> Result<Vec<(Key, Value)>, String> {
  let client = get_client().await;
 
  let data = match client.scan(Key::from(start_key)..Key::from(end_key), 10).await {
      Ok(data) => data,
      Err(err) => return Err(format!("Error scanning data: {}", err)),
  };
  let key_value_pairs: Vec<(Key, Value)> = data.into_iter().map(|kv_pair| (kv_pair.0,kv_pair.1)).collect();
  Ok(key_value_pairs)
}

pub async fn get_data(key: String) -> Result<String, String> {
  let client = get_client().await;

  match client.get(key).await {
      Ok(Some(value)) => {
          let final_value = String::from_utf8_lossy(&value).to_string();
          Ok(final_value)
      }
      Ok(None) => {
          Err("Data not found".to_string())
      }
      Err(err) => Err(format!("Error: {}", err)),
  }
}

pub async fn delete_data(key:String)->bool{
  let client = get_client().await;
 
    match client.delete(key.clone()).await {
      Ok(_) => {
           
          true
      }
      Err(_) => {
          
          false
      }
  }
}

pub async fn delete_data_range(skey:String,ekey:String)->bool{

  let client = get_client().await;

  match client.delete_range(skey..=ekey).await {
    Ok(_) => true, 
    Err(_) => false,
}
 
}

pub async fn put_data(key: String, value: String) -> Result<String, String> {
  let client = get_client().await;

  match client.put(key.clone(), value).await {
      Ok(_) => Ok(format!("Data stored successfully for key: {}", key)),
      Err(err) => Err(format!("Error storing data: {}", err)),
  }
}


// ----------------------------------------------------------------


 
// pub async fn get_client()-> Result<RawClient,String>{
 
  // loop {
    // tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    // match RawClient::new(vec!["127.0.0.1:2379"]).await {
        // Ok(client) => return Ok(client),
        // Err(err) => {
            // eprintln!("Error: failed to connect to db {}. Retrying...", err);
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        // }
    // }
// }
// }
 
// pub async fn delete_data(key: String) -> bool {
  // match get_client().await {
      // Ok(client) => {
          // match client.delete(key).await {
              // Ok(_) => true,
              // Err(_) => false,
          // }
      // }
      // Err(_) => false,
  // }
// }


// pub async fn delete_data_range(skey: String, ekey: String) -> bool {
  // match get_client().await {
      // Ok(client) => {
          // match client.delete_range(skey..=ekey).await {
              // Ok(_) => true,
              // Err(_) => false,
          // }
      // }
      // Err(_) => false,
  // }
// }
 
// pub async fn put_data(key: String, value: String) -> Result<String, String> {
  // match get_client().await {
      // Ok(client) => {
          // match client.put(key.clone(), value).await {
              // Ok(_) => Ok(format!("Data stored successfully for key: {}", key)),
              // Err(err) => Err(format!("Error storing data: {}", err)),
          // }
      // }
      // Err(_) => Err("Failed to get database connection".to_string()),
  // }
// }
 
// pub async fn scan_data(start_key: String, end_key: String) -> Result<Vec<(Key, Value)>, String> {
  // match get_client().await {
      // Ok(client) => {
          // match client.scan(Key::from(start_key)..Key::from(end_key), 10).await {
              // Ok(data) => {
                  // let key_value_pairs: Vec<(Key, Value)> = data.into_iter().map(|kv_pair| (kv_pair.0, kv_pair.1)).collect();
                  // Ok(key_value_pairs)
              // }
              // Err(err) => Err(format!("Error scanning data: {}", err)),
          // }
      // }
      // Err(_) => Err("Failed to get database connection".to_string()),
  // }
// }

// pub async fn get_data(key: String) -> Result<String, String> {
  // match get_client().await {
      // Ok(client) => {
          // match client.get(key).await {
              // Ok(Some(value)) => {
                  // let final_value = String::from_utf8_lossy(&value).to_string();
                  // Ok(final_value)
              // }
              // Ok(None) => Err("Data not found".to_string()),
              // Err(err) => Err(format!("Error: {}", err)),
          // }
      // }
      // Err(_) => Err("Failed to get database connection".to_string()),
  // }
// }
