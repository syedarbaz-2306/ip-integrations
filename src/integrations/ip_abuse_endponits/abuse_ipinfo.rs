use reqwest::Client;
use serde_json::Value;

pub async fn check_ip(ip_address: &str, api_key: &str) {
    let client = Client::new();
    let url = "https://api.abuseipdb.com/api/v2/check";

    // Perform the GET request with query parameters
    let response = client
        .get(url)
        .query(&[
            ("ipAddress", ip_address),
            ("maxAgeInDays", "90"),
            ("verbose", "true"),
        ])
        .header("Key", api_key)
        .header("Accept", "application/json")
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Value>().await {
                    Ok(json) => println!("Response JSON: {:#?}", json["data"]),
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

pub async fn fetch_reports(ip_address: &str, page: u32, per_page: u32, api_key: &str) {
    let client = Client::new();
    let url = "https://api.abuseipdb.com/api/v2/reports";

    // Perform the GET request with query parameters
    let response = client
        .get(url)
        .query(&[
            ("ipAddress", ip_address),
            ("page", &page.to_string()),
            ("perPage", &per_page.to_string()),
        ])
        .header("Key", api_key)
        .header("Accept", "application/json")
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

pub async fn fetch_blacklist(confidence_minimum: u32, api_key: &str, limit:u32) {
    let client = Client::new();
    let url = "https://api.abuseipdb.com/api/v2/blacklist?onlyCountries=IN,US,CA?ipVersion=6 ";

    // Perform the GET request with query parameters
    let response = client
        .get(url)
        .query(&[("confidenceMinimum", &confidence_minimum.to_string()),("limit",&limit.to_string())])
        .header("Key", api_key)
        .header("Accept", "application/json")
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
