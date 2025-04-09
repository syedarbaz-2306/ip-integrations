use reqwest::Method;
use base64::{encode as old_encode, Engine as _}; // Import Engine trait
use base64::engine::general_purpose::STANDARD as base64_engine;

use crate::integrations::{make_reqwest::{make_request, RequestConfig}, vt_url::types::UrlResponse};

pub async fn vt_get_url_report(url:String, api_key:String) {
    let url_bytes = url.as_bytes();
    let base64_string = base64_engine.encode(url_bytes);
    let base64_string_trimmed = base64_string.trim_end_matches('=');
    print!("{}",base64_string_trimmed);
    let final_url = format!("https://www.virustotal.com/api/v3/urls/{}", base64_string_trimmed,);

    let config = RequestConfig::new(final_url, Method::GET).headers(vec![
        ("x-apikey", api_key.clone()),
        ("accept", "application/json".to_string()),
    ]);

    let res = make_request(config).await;

    match res {
        Ok(Some(json))=>{
            let response = serde_json::from_value::<UrlResponse>(json)
            .map_err(|e| format!("Failed to parse response JSON: {}", e)).unwrap();
            print!("{:?}",response.data.into_action_response());
            
        }
        Ok(None)=>print!("succuess"),
        Err(e)=>print!("{}",e)
    }
}