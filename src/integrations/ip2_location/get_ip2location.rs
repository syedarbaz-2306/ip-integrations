use reqwest::Client;
use crate::integrations::{action_response::ActionResponse, ip2_location::types::Ip2Location};


pub async fn fetch_ip2loaction(
    ip_addr:&str, 
    api_key:&str,
)->Result<ActionResponse,String>{
 let client = Client::new();
 let url = format!("https://api.ip2location.io/?key={}&ip={}", api_key, ip_addr);

 let response = client.get(url).send().await;

 match response {
     Ok(result)=>{
        let data = result.json::<Ip2Location>().await.unwrap();
        let res = data.into_action_response();
        Ok(res)
     },
     Err(e)=>{
        Err(e.to_string())
     },
 }

}