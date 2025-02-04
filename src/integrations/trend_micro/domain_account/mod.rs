use serde::{Deserialize, Serialize};

use crate::integrations::{action_response::ActionResponse, into_action_response::IntoActionResponse};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DomainResponse {
    pub status: u16,
    #[serde(default)]
    pub headers: Option<Vec<Header>>,
    #[serde(default)]
    pub body: Option<ResponseBody>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseBody {
    pub error: ErrorInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
}

impl IntoActionResponse for Vec<DomainResponse> {
    fn into_action_response(self) -> ActionResponse {
        let mut action_response = ActionResponse::new();
        if let Some(object) = self.first() {
            action_response = action_response.set_output_field("status", object.status);
            if let Some(headers) = &object.headers {
                if let Some(header)= headers.get(0) {
                    action_response = action_response
                    .set_output_field("name", header.name.clone())
                    .set_output_field("value", header.value.clone());
                }
            }
            if let Some(body) = &object.body {
                action_response = action_response
                    .set_output_field("error_code", body.error.code.clone())
                    .set_output_field("error_message", body.error.message.clone());
            }
        }
        action_response
    }
}