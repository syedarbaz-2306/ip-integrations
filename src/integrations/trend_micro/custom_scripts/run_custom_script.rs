use serde::{Serialize, Deserialize};

use crate::integrations::action_response::ActionResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunCustomScript {
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ErrorResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

impl RunCustomScript {
    pub fn into_action_response(self)-> ActionResponse {
        let action_response = ActionResponse::new()
        .set_output_field("status", self.status);
    action_response
    }
}