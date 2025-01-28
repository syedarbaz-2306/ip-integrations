use reqwest::Client;
use crate::integrations::action_response::ActionResponse;

use super::types::IpinfoIs;

pub async fn fetch_ipinfo_is() -> Result<IpinfoIs, String> {
    let url = "https://ipinfo.is/";

    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| format!("Failed to send request: {}", err))?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response
        let json = response
            .json::<IpinfoIs>()
            .await
            .map_err(|err| format!("Failed to parse JSON response: {}", err))?;
        // let action_response = json.into_action_response();
        Ok(json)
    } else {
        // Handle HTTP errors
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Request failed message: {}",
            error_message
        ))
    }
}