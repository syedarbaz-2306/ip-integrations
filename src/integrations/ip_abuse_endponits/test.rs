use reqwest::Method;
use serde_json::Value;
use crate::integrations::{action_response::{self, ActionResponse}, ip_abuse_endponits::types::AbsCheckResponse, make_reqwest::{make_request, RequestConfig}};
pub async fn test_check_ip(
    ip_addr:String,
    api_key:String,
)-> Result<Option<ActionResponse>, String>{
    let config = RequestConfig::new("https://api.abuseipdb.com/api/v2/check", Method::GET)
    .params(vec![
        ("ipAddress", ip_addr),
    ])
    .headers(vec![
        ("Accept", "application/json"),
        ("Key",api_key.as_str())
    ]);

    let res = make_request(config).await;
    
    match res {
        Ok(Some(json))=>{
            match serde_json::from_value::<AbsCheckResponse>(json) {
                Ok(abs_check_response)=>Ok(Some(abs_check_response.into_action_response())),
                Err(e)=>Err(format!("Failed to parse response JSON: {}", e)),
            }
        },
        Ok(None)=>Ok(None),
        Err(e)=>Err(format!("error {}",e)),
    }
} 