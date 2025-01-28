use reqwest::Client;
use serde_json::Value;
use crate::integrations::action_response::ActionResponse;
use super::{model::InsightsResponse, types::{GeoIPResponseCity, GeoIPResponseCountry}};

pub async fn fetch_country_endpoint(
    ip_addr:&str,
    account_id:&str,
    license_key:&str,
)-> Result<GeoIPResponseCountry, String>{
    let client = Client::new();
    let url = format!("https://geoip.maxmind.com/geoip/v2.1/country/{}",ip_addr);
    let response = client
    .get(url)
    .basic_auth(account_id, Some(license_key))
    .send()
    .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<GeoIPResponseCountry>().await {
                    Ok(country_response) => {
                        // let action_response = country_response.into_action_response();
                        Ok(country_response)
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
        },
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

pub async fn fetch_city_endpoint(
    ip_addr:&str,
    account_id:&str,
    license_key:&str,
)-> Result<ActionResponse, String>{
    let client = Client::new();
    let url = format!("https://geoip.maxmind.com/geoip/v2.1/city/{}",ip_addr);
    let response = client
    .get(url)
    .basic_auth(account_id, Some(license_key))
    .send()
    .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<GeoIPResponseCity>().await {
                    Ok(city_response) => {
                        let action_response = city_response.into_action_response();
                        Ok(action_response)
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
        },
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


pub async fn fetch_insights_endpoint(
    ip_addr:&str,
    account_id:&str,
    license_key:&str,
)-> Result<InsightsResponse, String>{
    let client = Client::new();
    let url = format!("https://geoip.maxmind.com/geoip/v2.1/insights/{}",ip_addr);
    let response = client
    .get(url)
    .basic_auth(account_id, Some(license_key))
    .send()
    .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<InsightsResponse>().await {
                    Ok(insights_response ) => {
                        // let action_response = insights_response.into_action_response();
                        Ok(insights_response)
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
        },
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