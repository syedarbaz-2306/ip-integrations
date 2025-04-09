use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::integrations::action_response::ActionResponse;

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

impl IpAddressData {
    pub fn into_action_response(self) -> ActionResponse {
        
        let mut analysis_result:Vec<String> = Vec::new();
        for (key, value) in self.attributes.last_analysis_results.iter() {
            analysis_result.push(format!("{} result: {}",key, value.result));
        }

        let mut action_response = ActionResponse::new();
        action_response = action_response
            .set_output_field("id", self.id)
            .set_output_field("type", self.type_field)
            .set_output_field("self", self.links.self_url)
            .set_output_field("country", self.attributes.country)
            .set_output_field(
                "last_modification_date",
                self.attributes.last_modification_date,
            )
            .set_output_field("asn", self.attributes.asn)
            .set_output_field(
                "regional_internet_registry",
                self.attributes.regional_internet_registry,
            )
            .set_output_field("last_analysis_date", self.attributes.last_analysis_date)
            .set_output_field("continent", self.attributes.continent)
            .set_output_field("reputation", self.attributes.reputation)
            .set_output_field("network", self.attributes.network)
            .set_output_field("whois_date", self.attributes.whois_date)
            .set_output_field("whois", self.attributes.whois)
            .set_output_field("as_owner", self.attributes.as_owner)
            .set_output_field("tags", self.attributes.tags.join(", "))
            .set_output_field("total_votes_harmless", self.attributes.total_votes.harmless)
            .set_output_field(
                "total_votes_malicious",
                self.attributes.total_votes.malicious,
            )
            .set_output_field(
                "analysis_stats_malicious",
                self.attributes.last_analysis_stats.malicious,
            )
            .set_output_field(
                "analysis_stats_suspicious",
                self.attributes.last_analysis_stats.suspicious,
            )
            .set_output_field(
                "analysis_stats_undetected",
                self.attributes.last_analysis_stats.undetected,
            )
            .set_output_field(
                "analysis_stats_harmless",
                self.attributes.last_analysis_stats.harmless,
            )
            .set_output_field(
                "analysis_stats_timeout",
                self.attributes.last_analysis_stats.timeout,
            )
            .set_output_field("last_analysis_results", analysis_result.join(",\n"));
        action_response
    }
}
