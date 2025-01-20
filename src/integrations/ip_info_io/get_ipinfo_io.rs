use reqwest::Client;
use serde_json::Value;

pub async fn fetch_ip_info_io(
    base_url: &str,
    ip_address: &str,
    token: &str,
) {
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
                match resp.json::<Value>().await {
                    Ok(json) => println!("Response JSON: {:#?}", json),
                    Err(err) => eprintln!("Failed to parse JSON response: {}", err),
                }
            } else {
                eprintln!(
                    "Request failed with status: {} and message: {}",
                    resp.status(),
                    resp.text().await.unwrap_or_else(|_| "Unknown error".to_string())
                );
            }
        }
        Err(err) => match err.status() {
            Some(status) => eprintln!("Error: {} with status code: {}", err, status),
            None => eprintln!("Error: {}", err),
        },
    }
}
