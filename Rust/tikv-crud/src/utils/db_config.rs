use std::error::Error;


use tikv_client::{Key, RawClient,  Value};

pub async fn scan_data(start_key: String, end_key: String) -> Result<Vec<(Key, Value)>, Box<dyn Error>> {
  let client = get_client();
  let data = client.await.scan(Key::from(start_key)..Key::from(end_key), 10).await.map_err(|err| err)?;


   
    let key_value_pairs: Vec<(Key, Value)> = data.into_iter().map(|kv_pair| (kv_pair.0,kv_pair.1)).collect();

  Ok(key_value_pairs)
}

pub async fn get_client()->RawClient{
  RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap()

}


pub async fn get_data(key:String)-> Result<String , Box::<dyn Error>>{
  let client = get_client().await;

  let value = client.get(key).await.map_err(|err| err)?;
  let value = value.ok_or_else(||{println!("internal server error")}).unwrap();
  let finalvalue = String::from_utf8(value).map_err(|err|err)?;
  Ok(finalvalue)

  // match client.await.get(key.clone()).await.unwrap(){
  //   Some(value)=>{
  //     String::from_utf8(value)
  //   },
  //   None=>{
  //     // println!("key not matched: {:?}",key);
  //     Ok(format!("key not matched: {:?}",key))
    
  //   }

}



pub async fn delete_data(key:String){
  let client = get_client();

    client.await.delete(key.to_string()).await.unwrap()
   
}



pub async fn put_data(key: String, value: String) -> Result<String, Box<dyn Error>> {
  let client = get_client();
  client.await.put(key, value).await.map_err(|err| err)?;
  Ok("Data stored successfully".to_string())
}

