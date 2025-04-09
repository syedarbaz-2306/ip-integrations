use reqwest::Method;

use crate::integrations::make_reqwest::{make_request, RequestConfig};

use super::types::IpAddressResponse;

pub async fn vt_get_ip_report(ip_address:String, api_key:String) {
    let url = format!(
        "https://www.virustotal.com/api/v3/ip_addresses/{}",
        ip_address,
    );

    let config = RequestConfig::new(url, Method::GET).headers(vec![
        ("x-apikey", api_key),
        ("accept", "application/json".to_string()),
    ]);

    let res = make_request(config).await;

    match res {
        Ok(Some(json))=>{
            let response = serde_json::from_value::<IpAddressResponse>(json)
            .map_err(|e| format!("Failed to parse response JSON: {}", e)).unwrap();
            print!("{:?}",response.data.into_action_response());
            
        }
        Ok(None)=>print!("succuess"),
        Err(e)=>print!("{}",e)
    }
}