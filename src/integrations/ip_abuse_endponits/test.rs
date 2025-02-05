use reqwest::Method;
use crate::integrations::{ip_abuse_endponits::types::AbsCheckResponse, make_reqwest::{make_request, RequestConfig}};
pub async fn test_check_ip(
    ip_addr:String,
    api_key:String,
){
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
            println!("response:{:?}",json);
            match serde_json::from_value::<AbsCheckResponse>(json) {
                Ok(abs_check_response)=>println!("abs check struct {:?}",abs_check_response),
                Err(e)=>println!("error parsing json:{}",e),
            }
        },
        Ok(None)=>println!("success no content"),
        Err(e)=>println!("error! {:?}",e),
    }
} 