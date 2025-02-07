use reqwest::Method;
use serde_json::from_value;
use crate::integrations::{action_response::ActionResponse, ip_abuse_endponits::types::AbsCheckResponse, make_reqwest::{make_request, RequestConfig}};
pub async fn test_check_ip(
    ip_addr:String,
    api_key:String,
)-> Result<ActionResponse, String>{
    let config = RequestConfig::new("https://api.abuseipdb.com/api/v2/check", Method::GET)
    .params(vec![
        ("ipAddress", ip_addr),
    ])
    .headers(vec![
        ("Accept", "application/json"),
        ("Key",api_key.as_str())
    ]);

    let res = make_request(config).await;
    let action_response = ActionResponse::new();
    match res {
        Ok(Some(value))=>{
            match from_value::<AbsCheckResponse>(value) {
                Ok(r)=>Ok(r.into_action_response()),
                Err(e)=>Err(format!("Failed to parse response JSON: {}", e)),
            }
        },
        Ok(None)=>Ok(action_response),
        Err(e)=>Err(format!("error {}",e)),
    }
} 