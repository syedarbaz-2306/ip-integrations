// use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use crate::integrations::{action_response::{self, ActionResponse}, ip_stack::types::IpStack};

// // Error response structure
// pub async fn fetch_ipstack_info(
//     ip_address: &str,
//     access_key: &str,
// ) -> Result<IpStack, String> {
//     let client = Client::new();
//     let url = format!("http://api.ipstack.com/{}?access_key={}", ip_address, access_key);

//     let response = client.get(&url).send().await;
    
//     match response {
//         Ok(resp) => {
//             let text = resp.text().await.map_err(|e| format!("Failed to get response text: {}", e))?;
            
//             match serde_json::from_str::<IpStackResponse>(&text) {
//                 Ok(ip_stack_response) => {
//                     match ip_stack_response {
//                         IpStackResponse::Success(ipstack) => {
//                             // let action_response = ipstack.into_action_response();
//                             Ok(ipstack)
//                         },
//                         IpStackResponse::Error(error) => {
//                             Err(format!(
//                                 "IPStack API Error: {} (Code: {}, Type: {})",
//                                 error.error.info,
//                                 error.error.code,
//                                 error.error.error_type
//                             ))
//                         }
//                     }
//                 },
//                 Err(e) => {
//                     // Include the response text in the error for debugging
//                     Err(format!("Failed to parse response. Error: {}. Response text: {}", e, text))
//                 }
//             }
//         },
//         Err(err) => match err.status() {
//             Some(status) => {
//                 Err(format!("HTTP Error: {} with status code: {}", err, status))
//             },
//             None => {
//                 Err(format!("Network Error: {}", err))
//             },
//         },
//     }
// }