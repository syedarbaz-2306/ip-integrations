use chrono::format::Item;
use serde::{Deserialize, Serialize};

use crate::integrations::{action_response::{self, ActionResponse}, into_action_response::IntoActionResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct SuspiciousObject {
    #[serde(rename = "ip")]
    pub ip: Option<String>,
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "fileSha256")]
    pub file_sha256: Option<String>,
    #[serde(rename = "fileSha1")]
    pub file_sha1: Option<String>,
    #[serde(rename = "senderMailAddress")]
    pub sender_mail_address: Option<String>,
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub object_type: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "scanAction")]
    pub scan_action: String,
    #[serde(rename = "riskLevel")]
    pub risk_level: String,
    #[serde(rename = "inExceptionList")]
    pub in_exception_list: bool,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: String,
    #[serde(rename = "expiredDateTime")]
    pub expired_date_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuspiciousObjectResponse {
    pub items: Vec<SuspiciousObject>,
    #[serde(rename = "nextLink")]
    pub next_link: String,
}

impl IntoActionResponse for SuspiciousObjectResponse {
    fn into_action_response(self) -> ActionResponse {
        let mut action_response = ActionResponse::new();
        if let Some(items) = self.items.get(0) {
            action_response = action_response
                .set_output_field("ip", items.ip.clone().unwrap_or_default())
                .set_output_field("domain", items.domain.clone().unwrap_or_default())
                .set_output_field("file_sha256", items.file_sha256.clone().unwrap_or_default())
                .set_output_field("file_sha1", items.file_sha1.clone().unwrap_or_default())
                .set_output_field("sender_mail_address", items.sender_mail_address.clone().unwrap_or_default())
                .set_output_field("url", items.url.clone().unwrap_or_default())
                .set_output_field("object_type", items.object_type.clone())
                .set_output_field("description", items.description.clone().unwrap_or_default())
                .set_output_field("scan_action", items.scan_action.clone())
                .set_output_field("risk_level", items.risk_level.clone())
                .set_output_field("in_exception_list", items.in_exception_list.to_string())
                .set_output_field("last_modified_date_time", items.last_modified_date_time.clone())
                .set_output_field("expired_date_time", items.expired_date_time.clone());
        }
        action_response = action_response.set_output_field("next_link", self.next_link);
        action_response
    }
}