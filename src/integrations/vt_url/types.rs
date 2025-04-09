use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::integrations::action_response::ActionResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlResponse {
    pub data: UrlData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlData {
    pub attributes: Attributes,
    pub id: String,
    pub links: Links,
    #[serde(rename = "type")]
    pub data_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub categories: Option<HashMap<String, String>>,
    pub favicon: Option<Favicon>,
    #[serde(rename = "first_submission_date")]
    pub first_submission_date: Option<i64>,
    #[serde(rename = "html_meta")]
    pub html_meta: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "last_analysis_date")]
    pub last_analysis_date: Option<i64>,
    #[serde(rename = "last_analysis_results")]
    pub last_analysis_results: Option<HashMap<String, ScannerResult>>,
    #[serde(rename = "last_analysis_stats")]
    pub last_analysis_stats: Option<AnalysisStats>,
    #[serde(rename = "last_final_url")]
    pub last_final_url: Option<String>,
    #[serde(rename = "last_http_response_code")]
    pub last_http_response_code: Option<i64>,
    #[serde(rename = "last_http_response_content_length")]
    pub last_http_response_content_length: Option<i64>,
    #[serde(rename = "last_http_response_content_sha256")]
    pub last_http_response_content_sha256: Option<String>,
    #[serde(rename = "last_http_response_cookies")]
    pub last_http_response_cookies: Option<HashMap<String, String>>,
    #[serde(rename = "last_http_response_headers")]
    pub last_http_response_headers: Option<HashMap<String, String>>,
    #[serde(rename = "last_modification_date")]
    pub last_modification_date: Option<i64>,
    #[serde(rename = "last_submission_date")]
    pub last_submission_date: Option<i64>,
    #[serde(rename = "outgoing_links")]
    pub outgoing_links: Option<Vec<String>>,
    #[serde(rename = "redirection_chain")]
    pub redirection_chain: Option<Vec<String>>,
    pub reputation: Option<i64>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "targeted_brand")]
    pub targeted_brand: Option<HashMap<String, String>>,
    #[serde(rename = "times_submitted")]
    pub times_submitted: Option<i64>,
    pub title: Option<String>,
    #[serde(rename = "total_votes")]
    pub total_votes: Option<TotalVotes>,
    pub trackers: Option<HashMap<String, Vec<Tracker>>>,
    pub url: Option<String>,
    /// Added optional field for "tld" (top level domain) referenced in the builder.
    pub tld: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Favicon {
    pub dhash: Option<String>,
    #[serde(rename = "raw_md5")]
    pub raw_md5: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScannerResult {
    pub category: Option<String>,
    #[serde(rename = "engine_name")]
    pub engine_name: Option<String>,
    pub method: Option<String>,
    pub result: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AnalysisStats {
    pub harmless: Option<i64>,
    pub malicious: Option<i64>,
    pub suspicious: Option<i64>,
    pub timeout: Option<i64>,
    pub undetected: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TotalVotes {
    pub harmless: Option<i64>,
    pub malicious: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracker {
    pub id: Option<String>,
    pub timestamp: Option<i64>,
    pub url: Option<String>,
}

impl UrlData {
    pub fn into_action_response(self) -> ActionResponse {
        let atr = self.attributes;

        // Build outgoing links.
        let outgoing_links = atr
            .outgoing_links
            .unwrap_or_default()
            .join(",\n");

        // Process analysis results.
        let analysis_results = atr.last_analysis_results.unwrap_or_default();
        let analysis_results_vec: Vec<String> = analysis_results
            .iter()
            .map(|(key, value)| {
                format!(
                    "{} result: {}",
                    key,
                    value.result.clone().unwrap_or_default()
                )
            })
            .collect();

        // Process trackers.
        let trackers = atr.trackers.unwrap_or_default();
        let mut trackers_vec: Vec<String> = Vec::new();
        for (key, tracker_list) in trackers {
            for tracker in tracker_list {
                trackers_vec.push(format!(
                    "{}: URL: {}, ID: {}, Timestamp: {}",
                    key,
                    tracker.url.unwrap_or_default(),
                    tracker.id.unwrap_or_default(),
                    tracker.timestamp.unwrap_or(0)
                ));
            }
        }

        // Process html_meta.
        let html_meta = atr.html_meta.unwrap_or_default();
        let html_meta_vec: Vec<String> = html_meta
            .iter()
            .map(|(key, values)| format!("{}: {}", key, values.join(", ")))
            .collect();

        // Process HTTP response headers.
        let http_response_headers = atr.last_http_response_headers.unwrap_or_default();
        let http_response_headers_vec: Vec<String> = http_response_headers
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect();

        // Process categories.
        let categories = atr.categories.unwrap_or_default();
        let categories_vec: Vec<String> = categories
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect();

        // Use default values for nested optional structs.
        let analysis_stats = atr.last_analysis_stats.unwrap_or_default();
        let total_votes = atr.total_votes.unwrap_or_default();

        // Build the action response by unwrapping each optional value,
        // defaulting to "" for Strings and 0 for numbers if the field is None.
        let mut action_response = ActionResponse::new();
        action_response = action_response
            .set_output_field("id", self.id)
            .set_output_field("type", self.data_type)
            .set_output_field("self", self.links.self_url.unwrap_or_default())
            .set_output_field("outgoing_links", outgoing_links)
            .set_output_field("times_submitted", atr.times_submitted.unwrap_or(0))
            .set_output_field("title", atr.title.unwrap_or_default())
            .set_output_field("last_analysis_results", analysis_results_vec.join(",\n"))
            .set_output_field(
                "last_http_response_content_sha256",
                atr.last_http_response_content_sha256.unwrap_or_default(),
            )
            .set_output_field("reputation", atr.reputation.unwrap_or(0))
            .set_output_field("last_submission_date", atr.last_submission_date.unwrap_or(0))
            .set_output_field("first_submission_date", atr.first_submission_date.unwrap_or(0))
            .set_output_field("last_modification_date", atr.last_modification_date.unwrap_or(0))
            .set_output_field(
                "redirection_chain",
                atr.redirection_chain.unwrap_or_default().join(",\n"),
            )
            .set_output_field(
                "last_http_response_content_length",
                atr.last_http_response_content_length.unwrap_or(0),
            )
            .set_output_field("tags", atr.tags.unwrap_or_default().join(",\n"))
            .set_output_field("trackers", trackers_vec.join(",\n"))
            .set_output_field(
                "last_analysis_stats_malicious",
                analysis_stats.malicious.unwrap_or(0),
            )
            .set_output_field(
                "last_analysis_stats_suspicious",
                analysis_stats.suspicious.unwrap_or(0),
            )
            .set_output_field(
                "last_analysis_stats_undetected",
                analysis_stats.undetected.unwrap_or(0),
            )
            .set_output_field(
                "last_analysis_stats_harmless",
                analysis_stats.harmless.unwrap_or(0),
            )
            .set_output_field(
                "last_analysis_stats_timeout",
                analysis_stats.timeout.unwrap_or(0),
            )
            .set_output_field("html_meta", html_meta_vec.join(",\n"))
            .set_output_field("last_http_response_code", atr.last_http_response_code.unwrap_or(0))
            .set_output_field("last_analysis_date", atr.last_analysis_date.unwrap_or(0))
            .set_output_field("last_final_url", atr.last_final_url.unwrap_or_default())
            .set_output_field("tld", atr.tld.unwrap_or_default())
            .set_output_field(
                "last_http_response_headers",
                http_response_headers_vec.join(",\n"),
            )
            .set_output_field("categories", categories_vec.join(",\n"))
            .set_output_field("url", atr.url.unwrap_or_default())
            .set_output_field(
                "total_votes_harmless",
                total_votes.harmless.unwrap_or(0),
            )
            .set_output_field(
                "total_votes_malicious",
                total_votes.malicious.unwrap_or(0),
            );

        action_response
    }
}
