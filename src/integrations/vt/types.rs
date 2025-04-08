use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct IpAddressResponse {
    pub data: IpAddressData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpAddressData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub links: Link,
    pub attributes: Attributes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    #[serde(rename = "last_analysis_stats")]
    pub last_analysis_stats: AnalysisStats,
    pub asn: u32,
    #[serde(rename = "last_analysis_results")]
    pub last_analysis_results: HashMap<String, AnalysisResult>,
    #[serde(rename = "last_analysis_date")]
    pub last_analysis_date: u64,
    pub continent: String,
    #[serde(rename = "regional_internet_registry")]
    pub regional_internet_registry: String,
    pub country: String,
    pub reputation: i32,
    pub network: String,
    #[serde(rename = "whois_date")]
    pub whois_date: u64,
    #[serde(rename = "total_votes")]
    pub total_votes: TotalVotes,
    pub whois: String,
    #[serde(rename = "last_modification_date")]
    pub last_modification_date: u64,
    pub tags: Vec<String>,
    #[serde(rename = "as_owner")]
    pub as_owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisStats {
    pub malicious: u32,
    pub suspicious: u32,
    pub undetected: u32,
    pub harmless: u32,
    pub timeout: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisResult {
    pub method: String,
    pub engine_name: String,
    pub category: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalVotes {
    pub harmless: u32,
    pub malicious: u32,
}
