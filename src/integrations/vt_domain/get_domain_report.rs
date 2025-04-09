use reqwest::Method;

use crate::integrations::{make_reqwest::{make_request, RequestConfig}, vt_domain::types::DomainResponse};

pub async fn vt_get_domain_report(domain:String, api_key:String) {
    let url = format!("https://www.virustotal.com/api/v3/domains/{}", domain,);

    let config = RequestConfig::new(url, Method::GET).headers(vec![
        ("x-apikey", api_key.clone()),
        ("accept", "application/json".to_string()),
    ]);

    let res = make_request(config).await;

    match res {
        Ok(Some(json))=>{
            let response = serde_json::from_value::<DomainResponse>(json)
            .map_err(|e| format!("Failed to parse response JSON: {}", e)).unwrap();
            print!("{:?}",response.data.into_action_response());
            
        }
        Ok(None)=>print!("succuess"),
        Err(e)=>print!("{}",e)
    }
}