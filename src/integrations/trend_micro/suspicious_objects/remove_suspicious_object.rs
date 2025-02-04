use reqwest::Body;
use serde::{Serialize,Deserialize};

use crate::integrations::{action_response::ActionResponse, into_action_response::IntoActionResponse};

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoveSuspiciousObject {
    Url(String),
    Domain(String),
    Ip(String),
    SenderMailAddress(String),
    FileSha1(String),
    FileSha256(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseBody {
    pub error: Error,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ResponseBody>,
}

impl IntoActionResponse for Vec<Response> {
    fn into_action_response(self) -> ActionResponse {
        let mut action_response = ActionResponse::new();
        if let Some(object)= self.get(0) {
            action_response = action_response
            .set_output_field("status", object.status);
            if let Some(body)= &object.body {
                action_response = action_response
                .set_output_field("code", body.error.code.clone())
                .set_output_field("message", body.error.message.clone());
            }
        };
        action_response
    }
}

