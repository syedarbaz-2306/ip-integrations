use serde::{Deserialize, Serialize};

use crate::integrations::{action_response::ActionResponse, into_action_response::IntoActionResponse};

//? abs_check
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

impl IntoActionResponse for AbsCheckResponse {
    fn into_action_response(self)-> ActionResponse {
        let action_response = ActionResponse::new()
        .set_output_field("ipAddress", self.data.ip_address)
        .set_output_field("isPublic", self.data.is_public)
        .set_output_field("ipVersion", self.data.ip_version)
        .set_output_field("isWhitelisted", self.data.is_whitelisted)
        .set_output_field("abuseConfidenceScore", self.data.abuse_confidence_score)
        .set_output_field("countryCode", self.data.country_code)
        .set_output_field("usageType", self.data.usage_type)
        .set_output_field("isp", self.data.isp)
        .set_output_field("domain", self.data.domain)
        .set_output_field("hostnames", self.data.hostnames)
        .set_output_field("isTor", self.data.is_tor)
        .set_output_field("totalReports", self.data.total_reports)
        .set_output_field("numDistinctUsers", self.data.num_distinct_users)
        .set_output_field("lastReportedAt", self.data.last_reported_at);
    action_response
    }
}

//? abs_reports
#[derive(Debug, Serialize, Deserialize)]
pub struct AbsReportsResponse {
    pub data: ReportsResponse
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportsResponse {
    pub total: i32,
    pub page: i32,
    pub count: i32,
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "lastPage")]
    pub last_page: i32,
    #[serde(rename = "nextPageUrl")]
    pub next_page_url: Option<String>,
    #[serde(rename = "previousPageUrl")]
    pub previous_page_url: Option<String>,
    pub results: Vec<Reports>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reports {
    #[serde(rename = "reportedAt")]
    pub reported_at: String,
    pub comment: String,
    pub categories: Vec<i32>,
    #[serde(rename = "reporterId")]
    pub reporter_id: i32,
    #[serde(rename = "reporterCountryCode")]
    pub reporter_country_code: String,
    #[serde(rename = "reporterCountryName")]
    pub reporter_country_name: String,
}


impl IntoActionResponse for AbsReportsResponse {
    fn into_action_response(self)-> ActionResponse {
        let mut action_response = ActionResponse::new()
        .set_output_field("total", self.data.total)
        .set_output_field("page", self.data.page)
        .set_output_field("count", self.data.count)
        .set_output_field("per_page", self.data.per_page)
        .set_output_field("last_page", self.data.last_page)
        .set_output_field("next_page_url", self.data.next_page_url)
        .set_output_field("previous_page_url", self.data.previous_page_url);
        
        if let Some(first_result) = self.data.results.get(0) {
            action_response = action_response
                .set_output_field("reportedAt", first_result.reported_at.clone())
                .set_output_field("comment", first_result.comment.clone())
                .set_output_field("categories", first_result.categories.clone())
                .set_output_field("reporterId", first_result.reporter_id.clone())
                .set_output_field("reporterCountryCode", first_result.reporter_country_code.clone())
                .set_output_field("reporterCountryName", first_result.reporter_country_name.clone());
        };
    action_response
    }
}


//? abs_blacklist
#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpData {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "abuseConfidenceScore")]
    pub abuse_confidence_score: u8, // Assuming it's always between 0 and 100
    #[serde(rename = "lastReportedAt")]
    pub last_reported_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbsBlacklistResponse {
    pub meta: Meta,
    pub data: Vec<IpData>,
}

impl IntoActionResponse for AbsBlacklistResponse {
    fn into_action_response(self)-> ActionResponse {
        let action_response = ActionResponse::new()
        .set_output_field("generatedAt", self.meta.generated_at.clone())
        .set_output_field("ipAddress", self.data[0].ip_address.clone())
        .set_output_field("countryCode", self.data[0].country_code.clone())
        .set_output_field("abuseConfidenceScore", self.data[0].abuse_confidence_score.clone())
        .set_output_field("lastReportedAt", self.data[0].last_reported_at.clone());
    action_response
    }
}


//? abs report ip 
#[derive(Debug, Serialize, Deserialize)]
pub struct AbsReportResponse {
    pub data: ReportData,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ReportData {
    #[serde(rename = "ipAddress")]
    ip_address: String,
    #[serde(rename = "abuseConfidenceScore")]
    abuse_confidence_score: u32,
}

impl IntoActionResponse for AbsReportResponse {
    fn into_action_response(self)-> ActionResponse {
        let action_response = ActionResponse::new()
        .set_output_field("ipAddress", self.data.ip_address)
        .set_output_field("abuseConfidenceScore", self.data.abuse_confidence_score);
    action_response
    }
}