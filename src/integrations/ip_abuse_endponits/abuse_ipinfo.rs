use std::collections::HashMap;
use chrono::Utc;
use reqwest::Client;
use serde_json::Value;
use crate::integrations::{action_response::ActionResponse, ip_abuse_endponits::types::AbsReportResponse};

use super::types::{AbsBlacklistResponse, AbsCheck, AbsCheckResponse, AbsReportsResponse, ReportsResponse};

pub async fn check_ip(ip_address: &str, api_key: &str)->Result<AbsCheckResponse,String> {
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
                match resp.json::<AbsCheckResponse>().await {
                    Ok(abs_check) => {
                        // let action_response =  abs_check.data.into_action_response();
                        Ok(abs_check)
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

pub async fn fetch_reports(
    ip_address: &str,
    page: u32,
    per_page: u32,
    api_key: &str,
) -> Result<AbsReportsResponse, String> {
    let client = Client::new();
    let url = "https://api.abuseipdb.com/api/v2/reports";

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
                match resp.json::<AbsReportsResponse>().await {
                    Ok(api_response) => {
                        Ok(api_response)
                    },
                    Err(err) => Err(format!("Failed to get response text: {}", err))
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
            Some(status) => Err(format!("Error: {} with status code: {}", err, status)),
            None => Err(format!("Error: {}", err)),
        },
    }
}

pub async fn fetch_blacklist(
    confidence_minimum: u32, 
    api_key: &str, 
    limit:u32,
) -> Result<AbsBlacklistResponse, String> {
    let client = Client::new();
    let url = format!("https://api.abuseipdb.com/api/v2/blacklist");

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
                match resp.json::<AbsBlacklistResponse>().await {
                    Ok(json) => {
                        Ok(json)
                    },
                    Err(err) => Err(format!("Failed to parse JSON response: {}", err)),
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
            Some(status) => Err(format!("Error: {} with status code: {}", err, status)),
            None => Err(format!("Error: {}", err)),
        },
    }
}


pub async fn report_ip(ip:&str, api_key: &str, categories:&str, comment: &str)-> Result<AbsReportResponse, String>{
    let client = Client::new();
    let url = "https://api.abuseipdb.com/api/v2/report";

    let mut form = HashMap::new();
    let current_datetime = Utc::now().to_rfc3339();
    println!("current datetime: {}", current_datetime);
    form.insert("ip", ip);
    form.insert("categories", categories);
    form.insert("comment", comment);
    form.insert("timestamp", &current_datetime);

     let result = client
        .post(url)
        .form(&form)
        .header("Key", api_key)
        .header("Accept", "application/json")
        .send()
        .await;

    match result {
        Ok(response)=> {
            let body = response.json::<AbsReportResponse>().await;
            match body {
                Ok(json)=> {
                    Ok(json)
                },
                Err(err)=> Err(format!("Error parsing JSON: {}", err)),
            }
        },
        Err(err) => Err(format!("error {}",err)),
    }
}
