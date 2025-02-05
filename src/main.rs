mod integrations;
use std::collections::HashMap;
use std::{env, vec};

use dotenv::dotenv;
use integrations::ip2_location::get_ip2location::fetch_ip2loaction;
use integrations::ip2_location::types::Ip2Location;
use integrations::ip_abuse_endponits::abuse_ipinfo::{
    check_ip, fetch_blacklist, fetch_reports, report_ip,
};
use integrations::ip_abuse_endponits::test::test_check_ip;
use integrations::ip_abuse_endponits::types::{
    AbsBlacklistResponse, AbsCheckResponse, AbsReportResponse, AbsReportsResponse,
};
use integrations::ip_info_io::get_ipinfo_io::fetch_ip_info_io;
use integrations::ip_info_is::get_ip_info_is::fetch_ipinfo_is;
use integrations::ip_info_is::types::IpinfoIs;
use integrations::ip_stack::types::IpStack;
use integrations::make_reqwest::Auth::Basic;
use integrations::make_reqwest::{make_request, Auth::Bearer, RequestConfig};
use integrations::max_mind::get_max_mind::{
    fetch_city_endpoint, fetch_country_endpoint, fetch_insights_endpoint,
};
use integrations::max_mind::model::InsightsResponse;
use integrations::max_mind::types::{GeoIPResponseCity, GeoIPResponseCountry};
use integrations::trend_micro::custom_scripts::list_custom_scripts::ListCustomScriptsResponse;
use integrations::trend_micro::domain_account::DomainResponse;
use integrations::trend_micro::suspicious_objects::remove_suspicious_object::{RemoveSuspiciousObject, Response};
use integrations::trend_micro::suspicious_objects::{
    add_suspicious_object::{AddSuspiciousObjectResponse, SuspiciousBodyObject},
    suspicious_object_list::SuspiciousObjectResponse,
};
use reqwest::header::{self, HeaderMap, HeaderValue, ACCEPT};
use reqwest::Method;
use serde_json::{json, Value};

use integrations::trend_micro::suspicious_objects::add_suspicious_object::*;
use reqwest::multipart;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let ipinfo_io_token = env::var("IPINFO_IO").expect("IPINFO_IO not found in .env file");

    let abuseipdb_apikey =
        env::var("ABUSEIPDB_APIKEY").expect("ABUSEIPDB_APIKEY not found in .env file");

    let ipstack_apikey = env::var("IPSTACK_APIKEY").expect("IPSTACK_APIKEY not found in .env file");

    let ip2loaction_key =
        env::var("IP2LOCATION_KEY").expect("IP2LOCATION_KEY not found in .env file");

    let maxmind_accounid =
        env::var("MAXMIND_ACCOUNT_ID").expect("MAXMIND_ACCOUNT_ID not found in .env file");

    let maxmind_license_key =
        env::var("MAXMIND_LICENSE_KEY").expect("MAXMIND_LICENSE_KEY not found in .env file");

    let trend_micro_token =
        env::var("TREND_MICRO_TOKEN").expect("TREND_MICRO_TOKEN not found in .env file");

    // //* */ ip infois example
    // let config = RequestConfig::new("https://ipinfo.is/", Method::GET);
    // let res = make_request::<IpinfoIs>(config).await;

    // match res {
    //     Ok(action_response)=>println!("action response : {:?}", action_response),
    //     Err(e)=>println!("Error {}", e),
    // }

    // //* */ ip info io example
    // let ip_address = "2406:7400:9a:69a9:d7f:6040:91ac:b84f";
    // let config = RequestConfig{
    //     url:format!("https://ipinfo.io/{}?token={}",ip_address, token),
    //     params:None,
    //     method:Method::GET,
    //     headers:None,
    //     body:None,
    //     auth:None,
    // };

    // let res = make_request::<IpinfoIo>(config).await;

    // match res {
    //     Ok(action_response)=>println!("action response : {:?}", action_response),
    //     Err(e)=>println!("Error {}", e),
    // }

    // //* */ ip abuse check end points example
    // let ip_address = "2406:7400:9a:69a9:d7f:6040:91ac:b84f";

    // let body = Value::Object({
    //     let mut map = serde_json::Map::new();
    //     map.insert("ipAddress".to_string(), Value::String(ip_address.to_string()));
    //     map.insert("maxAgeInDays".to_string(), Value::String("90".to_string()));
    //     map
    // });

    // let mut headers = HeaderMap::new();
    // headers.insert("Key", HeaderValue::from_str(api_key).unwrap());
    // headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    // let config = RequestConfig::new("https://api.abuseipdb.com/api/v2/check".to_string(), Method::GET)
    // .with_body(body)
    // .with_headers(headers);

    // let res = make_request::<AbsCheckResponse>(config).await;

    // match res {
    //     Ok(action_response)=>println!("action response : {:?}", action_response),
    //     Err(e)=>println!("Error {}", e),
    // }

    //* */ ip abuse reports end point example
    // let ip_address = "2406:7400:9a:69a9:d7f:6040:91ac:b84f";

    // let mut params = HashMap::new();
    // params.insert("ipAddress".to_string(), ip_address.to_string());
    // params.insert("page".to_string(), "1".to_string());
    // params.insert("perPage".to_string(), "5".to_string());

    // let mut headers = HeaderMap::new();
    // headers.insert("Key", HeaderValue::from_str(api_key).unwrap());
    // headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    // let config = RequestConfig::new("https://api.abuseipdb.com/api/v2/reports", Method::GET)
    //     .with_params(params)
    //     .with_headers(headers);
    // let res = make_request::<AbsReportsResponse>(config).await;
    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //* */ ip abuse blacklist end point example

    // let mut params = HashMap::new();
    // params.insert("confidenceMinimum".to_string(), "90".to_string());

    // let mut header = HeaderMap::new();
    // header.insert(ACCEPT, HeaderValue::from_static("application/json"));
    // header.insert("key", HeaderValue::from_static(api_key));

    // let config = RequestConfig::new("https://api.abuseipdb.com/api/v2/blacklist".to_string(), Method::GET)
    // .with_params(params)
    // .with_headers(header);

    // let res = make_request::<AbsBlacklistResponse>(config).await;
    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    // //* */ ip abuse report endpoint
    // let ip_address = "2406:7400:9a:69a9:d7f:6040:91ac:b84f";
    // let comment = "SSH login attempts with user root.";
    // let timestamp = "2023-10-18T11:25:11-04:00";
    // let categories = "1";

    // let mut params = HashMap::new();
    // params.insert("ip".to_string(), ip_address.to_string());
    // params.insert("comment".to_string(), comment.to_string());
    // params.insert("timestamp".to_string(), timestamp.to_string());
    // params.insert("categories".to_string(), categories.to_string());

    // let mut header = HeaderMap::new();
    // header.insert(ACCEPT, HeaderValue::from_static("application/json"));
    // header.insert("key", HeaderValue::from_str(&abuseipdb_apikey));

    // let config = RequestConfig::new("https://api.abuseipdb.com/api/v2/report".to_string(), Method::POST)
    // .with_params(params)
    // .with_headers(header);

    // let res = make_request::<AbsReportResponse>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    // //* ipstack example */
    // let ip_addr = "2406:7400:9a:f9be:b1d8:dc23:8638:19a2";

    // let config = RequestConfig::new(
    //     format!("https://api.ipstack.com/{}?access_key={}", ip_addr, access_key),
    //     Method::GET,
    // );

    // let res = make_request::<IpStack>(config).await;
    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //*ip2 location example */
    // let ip_addr = "152.58.193.76";
    // let config = RequestConfig::new(
    //     format!("https://api.ip2location.io/?key={}&ip={}", api_key, ip_addr),
    //     Method::GET,
    // );
    // let res = make_request::<Ip2Location>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //* maxmind insights endpoint */
    // let ip_addr = "2406:7400:9a:69a9:d7f:6040:91ac:b84f";

    // let auth = Basic { username: account_id.to_string(), password: Some(maxmind.to_string()) };
    // let config = RequestConfig::new(format!("https://geoip.maxmind.com/geoip/v2.1/insights/{}",ip_addr), Method::GET)
    // .with_auth(auth);

    // let res = make_request::<InsightsResponse>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //* maxmind country endpoint */
    // let ip_addr = "2406:7400:9a:69a9:d7f:6040:91ac:b84f";

    // let auth = Basic { username: account_id.to_string(), password: Some(maxmind.to_string()) };
    // let config = RequestConfig::new(format!("https://geoip.maxmind.com/geoip/v2.1/country/{}",ip_addr), Method::GET)
    // .with_auth(auth);

    // let res = make_request::<GeoIPResponseCountry>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    // //* maxmind city endpoint */
    // let ip_addr = "2406:7400:9a:69a9:d7f:6040:91ac:b84f";

    // let auth = Basic { username: maxmind_accounid.to_string(), password: Some(maxmind_license_key.to_string()) };
    // let config = RequestConfig::new(format!("https://geoip.maxmind.com/geoip/v2.1/city/{}",ip_addr), Method::GET)
    // .with_auth(auth);

    // let res = make_request::<GeoIPResponseCity>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //* trend_micro list of suspicious objects */
    // let config = RequestConfig::new("https://api.xdr.trendmicro.com/v3.0/threatintel/suspiciousObjects", Method::GET)
    // .params(&[
    //     ("orderBy", "lastModifiedDateTime desc"),
    //     // ("startDateTime", "YOUR_STARTDATETIME (string)"),
    //     // ("endDateTime", "YOUR_ENDDATETIME (string)"),
    //     ("top", "50"),
    // ])
    // .headers(&[
    //     ("Authorization", format!("Bearer {}",trend_micro_token)),
    // ]);

    // let res = make_request::<SuspiciousObjectResponse>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //* add suspicious object */
    // let config = RequestConfig::new(
    //     "https://api.xdr.trendmicro.com/v3.0/threatintel/suspiciousObjects",
    //     Method::POST,
    // )
    // .json_body([
    //     SuspiciousBodyObject {
    //         object_type:SuspiciousObjectType::Url { url: "http://test.com".to_string() },
    //         description:"YOUR_DESCRIPTION (string)".to_string(),
    //         scanAction:"YOUR_SCANACTION (string)".to_string(),
    //         riskLevel:"YOUR_RISKLEVEL (string)".to_string(),
    //         daysToExpiration:50,
    //     },
    //     SuspiciousBodyObject {
    //         object_type:SuspiciousObjectType::Domain { domain: "YOUR_DOMAIN (string)".to_string() },
    //         description:"YOUR_DESCRIPTION (string)".to_string(),
    //         scanAction:"YOUR_SCANACTION (string)".to_string(),
    //         riskLevel:"YOUR_RISKLEVEL (string)".to_string(),
    //         daysToExpiration:50,
    //     }
    // ])
    // .headers(&[
    //     ("Authorization", format!("Bearer {}", trend_micro_token)),
    //     ("Content-Type", "application/json".to_string()),
    // ]);

    // let res = make_request::<Vec<AddSuspiciousObjectResponse>>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //*remove suspicious object */
    // let config = RequestConfig::new("https://api.xdr.trendmicro.com/v3.0/threatintel/suspiciousObjects/delete", Method::POST)
    // .json_body(
    //         json!(
    //             [
    //                 {
    //                 "url":RemoveSuspiciousObject::Url("http://test.com".to_string())
    //                 },
    //                 {
    //                 "domain":RemoveSuspiciousObject::Domain("http://test.com".to_string())
    //                 },
    //             ]
    //         )
    // ).headers(&[
    //     ("Authorization", format!("Bearer {}", trend_micro_token)),
    //     ("Content-Type", "application/json".to_string()),
    // ]);

    // let res = make_request::<Vec<Response>>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }


    //*trend-micro disable user account */
    // let config = RequestConfig::new("https://api.xdr.trendmicro.com/v3.0/response/domainAccounts/disable", Method::POST)
    // .json_body(
    //     json!(
    //         [
    //             {
    //                 "accountName": "YOUR_ACCOUNTNAME (string)",
    //                 "description": "YOUR_DESCRIPTION (string)",
    //             }
    //         ]
    //     )
    // )
    // .headers(&[
    //     ("Authorization", format!("Bearer {}", trend_micro_token)),
    //     ("Content-Type", "application/json;charset=utf-8".to_string()),
    // ]);
    
    // let res = make_request::<Vec<DomainResponse>>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //*trend-micro enable user account */
    // let config = RequestConfig::new("https://api.xdr.trendmicro.com/v3.0/response/domainAccounts/enable", Method::POST)
    // .json_body(
    //     json!(
    //         [
    //             {
    //                 "accountName": "YOUR_ACCOUNTNAME (string)",
    //                 "description": "YOUR_DESCRIPTION (string)",
    //             }
    //         ]
    //     )
    // )
    // .headers(&[
    //     ("Authorization", format!("Bearer {}", trend_micro_token)),
    //     ("Content-Type", "application/json;charset=utf-8".to_string()),
    // ]);
    
    // let res = make_request::<Vec<DomainResponse>>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //*trend-micro force signout */
    // let config = RequestConfig::new("https://api.xdr.trendmicro.com/v3.0/response/domainAccounts/signOut", Method::POST)
    // .json_body(
    //     json!(
    //         [
    //             {
    //                 "accountName": "YOUR_ACCOUNTNAME (string)",
    //                 "description": "YOUR_DESCRIPTION (string)",
    //             }
    //         ]
    //     )
    // )
    // .headers(&[
    //     ("Authorization", format!("Bearer {}", trend_micro_token)),
    //     ("Content-Type", "application/json;charset=utf-8".to_string()),
    // ]);
    
    // let res = make_request::<Vec<DomainResponse>>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //*trend-micro force password reset */
    // let config = RequestConfig::new("https://api.xdr.trendmicro.com/v3.0/response/domainAccounts/resetPassword", Method::POST)
    // .json_body(
    //     json!(
    //         [
    //             {
    //                 "accountName": "YOUR_ACCOUNTNAME (string)",
    //                 "description": "YOUR_DESCRIPTION (string)",
    //             }
    //         ]
    //     )
    // )
    // .headers(&[
    //     ("Authorization", format!("Bearer {}", trend_micro_token)),
    //     ("Content-Type", "application/json;charset=utf-8".to_string()),
    // ]);
    
    // let res = make_request::<Vec<DomainResponse>>(config).await;

    // match res {
    //     Ok(action_response) => println!("Action response: {:?}", action_response),
    //     Err(e) => println!("Error: {}", e),
    // }

    //*trend-micro list custom scripts */

    // let filename: Option<String> = None;
    // let filetype: Option<String> = None;

    // // Build query parameters
    // let mut params: Vec<(&str, String)> = Vec::new();

    // if let Some(ref file_name) = filename {
    //     params.push(("filter", format!("fileName eq '{}'", file_name)));
    // }

    // if let Some(ref file_type) = filetype {
    //     params.push(("filter", format!("fileType eq '{}'", file_type)));
    // }

    // // Build the request configuration
    // let config = RequestConfig::new(
    //     "https://api.xdr.trendmicro.com/v3.0/response/customScripts",
    //     Method::GET,
    // )
    // .params(params) // Add query parameters
    // .headers(vec![("Authorization", format!("Bearer {}", trend_micro_token))]); // Add headers

    // let res = make_request(config).await;

    // match res {
    //     Ok(Some(json)) => {
    //         match serde_json::from_value::<ListCustomScriptsResponse>(json) {
    //             Ok(response) => println!("res: {:?}",response),
    //             Err(e) => println!("Error parsing JSON: {}", e),
    //         }
    //     }
    //     Ok(None) => {
    //         println!("Success! No content returned (204).");
    //     }
    //     Err(error_json) => {
    //         println!("Error! API returned: {:?}", error_json);
    //     }
    // }


    //? abuseipdb check endpoint 
    let ip_addr = format!("2406:7400:9a:69a9:d7f:6040:91ac:b84f");
    test_check_ip(ip_addr, abuseipdb_apikey).await;
}
