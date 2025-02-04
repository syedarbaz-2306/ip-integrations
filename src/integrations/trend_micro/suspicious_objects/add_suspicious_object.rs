use serde::{Deserialize, Serialize};

use crate::integrations::{action_response::ActionResponse, into_action_response::IntoActionResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    pub error: ErrorResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddSuspiciousObjectResponse {
    pub status: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ResponseBody>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "objectType", rename_all = "camelCase")]
pub enum SuspiciousObjectType {
    Url { url: String },
    Domain { domain: String },
    SenderMailAddress { senderMailAddress: String },
    Ip { ip: String },
    FileSha1 { fileSha1: String },
    FileSha256 { fileSha256: String },
}

#[derive(Serialize, Deserialize)]
pub struct SuspiciousBodyObject {
    #[serde(flatten)]
    pub object_type: SuspiciousObjectType, // Enum that determines the object type
    pub description: String,
    pub scanAction: String,
    pub riskLevel: String,
    pub daysToExpiration: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SuspiciousObjectsRequest {
    suspiciousObjects: Vec<SuspiciousBodyObject>,
}



impl IntoActionResponse for Vec<AddSuspiciousObjectResponse> {
    fn into_action_response(self) -> ActionResponse {
        let mut action_response = ActionResponse::new();
        if let Some(api_response) = self.get(0){
            action_response = action_response
            .set_output_field("status", api_response.status);
            if let Some(body) = &api_response.body {
                action_response = action_response
                .set_output_field("code", body.error.code.clone())
                .set_output_field("message", body.error.message.clone());
            }
        }
        action_response
    }
}