use serde::{Deserialize, Serialize};

use crate::integrations::action_response::ActionResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct AbsCheckResponse {
    pub data: AbsCheck,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbsCheck {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "ipVersion")]
    pub ip_version: u8,
    #[serde(rename = "isWhitelisted")]
    pub is_whitelisted: Option<bool>,
    #[serde(rename = "abuseConfidenceScore")]
    pub abuse_confidence_score: u8,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "usageType")]
    pub usage_type: String,
    pub isp: String,
    pub domain: String,
    pub hostnames: Vec<String>,
    #[serde(rename = "isTor")]
    pub is_tor: bool,
    #[serde(rename = "totalReports")]
    pub total_reports: u32,
    #[serde(rename = "numDistinctUsers")]
    pub num_distinct_users: u32,
    #[serde(rename = "lastReportedAt")]
    pub last_reported_at: Option<String>,
}

impl AbsCheck {
    pub fn into_action_response(self)-> ActionResponse {
        let action_response = ActionResponse::new()
        .set_output_field("ipAddress", self.ip_address)
        .set_output_field("isPublic", self.is_public)
        .set_output_field("ipVersion", self.ip_version)
        .set_output_field("isWhitelisted", self.is_whitelisted)
        .set_output_field("abuseConfidenceScore", self.abuse_confidence_score)
        .set_output_field("countryCode", self.country_code)
        .set_output_field("usageType", self.usage_type)
        .set_output_field("isp", self.isp)
        .set_output_field("domain", self.domain)
        .set_output_field("hostnames", self.hostnames)
        .set_output_field("isTor", self.is_tor)
        .set_output_field("totalReports", self.total_reports)
        .set_output_field("numDistinctUsers", self.num_distinct_users)
        .set_output_field("lastReportedAt", self.last_reported_at);
    action_response
    }
}