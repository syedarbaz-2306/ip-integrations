use reqwest::Client;
use crate::integrations::action_response::{self, ActionResponse};

use super::response_type::IpinfoIo;

pub async fn fetch_ip_info_io(
    base_url: &str,
    ip_address: &str,
    token: &str,
)-> Result<IpinfoIo,String> {
    let client = Client::new();
    let url = format!("{}{}?token={}", base_url, ip_address, token);

    // Send the HTTP request
    let response = client
        .get(&url)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<IpinfoIo>().await {
                    Ok(ipinfoio) => {
                        // let action_response = ipinfoio.into_action_response();
                        Ok(ipinfoio)
                    },
                    Err(err) => {
                        let err = format!("Failed to parse JSON response: {}", err);
                        Err(err)
                    },
                }
            } else {
                let err = format!(
                    "Request failed with status: {} and message: {}",
                    resp.status(),
                    resp.text().await.unwrap_or_else(|_| "Unknown error".to_string())
                );
                Err(err)
            }
        }
        Err(err) => match err.status() {
            Some(status) => {
                let err = format!("Error: {} with status code: {}", err, status);
                Err(err)
            },
            None => {
                let err = format!("Error: {}", err);
                Err(err)
            },
        },
    }
}
