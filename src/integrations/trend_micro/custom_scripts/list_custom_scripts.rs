use serde::{Deserialize, Serialize};

use crate::integrations::{action_response::ActionResponse, into_action_response::IntoActionResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCustomScriptsResponse {
    pub items: Vec<CustomScript>,
    #[serde(rename = "nextLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomScript {
    pub id: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileType")]
    pub file_type: String,
    pub description: String,
}

impl IntoActionResponse for ListCustomScriptsResponse {
    fn into_action_response(self) -> ActionResponse {
        let mut action_response = ActionResponse::new();
        if let Some(items) = self.items.get(0){
            action_response = action_response
            .set_output_field("id", items.id.clone())
            .set_output_field("fileName", items.file_name.clone())
            .set_output_field("fileType", items.file_type.clone())
            .set_output_field("description", items.description.clone())
        }
        if let Some(next_link)= self.next_link{
            action_response = action_response
            .set_output_field("nextLink", next_link.clone())
        }
        action_response
    }
}