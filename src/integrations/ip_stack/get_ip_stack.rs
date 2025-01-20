use reqwest::Client;
use serde_json::Value;
use crate::integrations::{action_response::ActionResponse, ip_stack::types::*};


pub async fn fetch_ipstack_info(
    ip_address: &str,
    access_key: &str,
) -> Result<ActionResponse, String> {
    let client = Client::new();
    let url = format!("http://api.ipstack.com/{}?access_key={}", ip_address,access_key);

    // Send the HTTP request
    let response = client.get(&url).send().await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<IpStack>().await {
                    Ok(json) => {
                        let action_response = json.into_action_response();
                        Ok(action_response)
                    }
                    Err(err) => Err(format!("Failed to parse JSON response: {}", err)),
                }
            } else {
                Err(format!(
                    "Request failed with status: {} and message: {}",
                    resp.status(),
                    resp.text().await.unwrap_or_else(|_| "Unknown error".to_string())
                ))
            }
        }
        Err(err) => Err(format!("Network error: {}", err)),
    }
}
