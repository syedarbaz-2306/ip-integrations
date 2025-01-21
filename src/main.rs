
mod integrations;
use integrations::ip2_location::get_ip2location::fetch_ip2loaction;
use integrations::ip_info_is::get_ip_info_is::fetch_ipinfo_is;
// use crate::integrations::ip_info_io::get_ipinfo_io::fetch_ip_info_io;
// use integrations::ip_abuse_endponits::abuse_ipinfo::{check_ip, fetch_reports, fetch_blacklist};
use integrations::{action_response, ip_stack::get_ip_stack::fetch_ipstack_info};
#[tokio::main]
async fn main() {

    // //? ip info is example 
    // match fetch_ipinfo_is().await {
    //     Ok(res) => println!("data : {:?}",res),
    //     Err(e)=> println!("error : {}",e),
    // }

    //? ip info io example 
    // let base_url = "https://ipinfo.io/";
    // let ip_address = "2406:7400:9a:f9be:1dbd:7bad:7d0b:9d20";
    // let token = "45d800e5d0ba21";
    // fetch_ip_info_io(&base_url, &ip_address, &token).await;

    // //? ip abuse check end points example
    // let ip_address = "2406:7400:9a:f9be:1dbd:7bad:7d0b:9d20";
    // let api_key = "9adee9dd3e9bdaf9c519a9d03a828eaf87e41b1646f183cc62889b2337816601208c104db898f19c";
    // check_ip(ip_address, api_key).await;

    // //? ip abuse reports end point example
    // let ip_address = "118.172.234.158";
    // let page = 1;
    // let per_page = 25;
    // let api_key = "9adee9dd3e9bdaf9c519a9d03a828eaf87e41b1646f183cc62889b2337816601208c104db898f19c";
    // fetch_reports(ip_address, page, per_page, api_key).await;

    // //? ip abuse blacklist end point example
    // let confidence_minimum = 90;
    // let limit = 10;
    // let api_key = "9adee9dd3e9bdaf9c519a9d03a828eaf87e41b1646f183cc62889b2337816601208c104db898f19c";
    // fetch_blacklist(confidence_minimum, api_key, limit).await;

    // //? ip stack endpoint expample 
    // let ip_address = "2406:7400:9a:f9be:b1d8:dc23:8638:19a2";
    // let access_key = "872fdef7c24be197d8ef6ab203d3988a";

    // match fetch_ipstack_info(ip_address, access_key).await {
    //     Ok(action_response)=> println!("action response: {:?}", action_response),
    //     Err(e) => println!("Error {}", e),
    // };

    //? ip2 loaction endpoint example 

    let ip_addr = "152.58.193.76";
    let api_key = "7FBCD5101CA15C2A79C56A35F146E2DB";
    let res = fetch_ip2loaction(ip_addr, api_key).await;

    match res {
        Ok(action_response)=>println!("json:{:?}",action_response),
        Err(e) => println!("Error {}", e),
    }
}
