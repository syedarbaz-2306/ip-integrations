use std::{any::Any, collections::HashMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::integrations::{action_response::ActionResponse, into_action_response::IntoActionResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct IpStack {
    pub ip: String,  // Changed from Option<String> since it's always present
    #[serde(rename = "type")]
    pub ip_type: String,  // Changed name and removed Option since it's always present
    pub continent_code: String,
    pub continent_name: String,
    pub country_code: String,
    pub country_name: String,
    pub region_code: String,
    pub region_name: String,
    pub city: String,
    pub zip: String,
    pub latitude: f64,
    pub longitude: f64,
    pub msa: Option<Value>,
    pub dma: Option<Value>,
    pub radius: Option<Value>,
    #[serde(rename = "ip_routing_type")]
    pub ip_routing_type: String,
    pub connection_type: Option<Value>,
    pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub geoname_id: u64,
    pub capital: String,
    pub languages: Vec<Language>,
    pub country_flag: String,
    pub country_flag_emoji: String,
    pub country_flag_emoji_unicode: String,
    pub calling_code: String,
    pub is_eu: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub native: String,
}

#[derive(Debug, Deserialize)]
struct IpStackError {
    success: bool,
    error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ErrorDetail {
    code: i32,
    #[serde(rename = "type")]
    error_type: String,
    info: String,
}

// Combined response type
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum IpStackResponse {
    Success(IpStack),
    Error(IpStackError),
}


impl IntoActionResponse for IpStack {
    fn into_action_response(self) -> ActionResponse {
        let mut response = ActionResponse::new();

        // Base fields
        response = response
            .set_output_field("ip", self.ip)
            .set_output_field("type", self.ip_type)
            .set_output_field("continent_code", self.continent_code)
            .set_output_field("continent_name", self.continent_name)
            .set_output_field("country_code", self.country_code)
            .set_output_field("country_name", self.country_name)
            .set_output_field("region_code", self.region_code)
            .set_output_field("region_name", self.region_name)
            .set_output_field("city", self.city)
            .set_output_field("zip", self.zip)
            .set_output_field("latitude", self.latitude)
            .set_output_field("longitude", self.longitude)
            .set_output_field("ip_routing_type", self.ip_routing_type);

        // Optional Value fields
        if let Some(msa) = self.msa {
            response = response.set_output_field("msa", msa);
        }
        if let Some(dma) = self.dma {
            response = response.set_output_field("dma", dma);
        }
        if let Some(radius) = self.radius {
            response = response.set_output_field("radius", radius);
        }
        if let Some(connection_type) = self.connection_type {
            response = response.set_output_field("connection_type", connection_type);
        }

        // Location fields
        let mut location_map = HashMap::new();
        location_map.insert("geoname_id".to_string(), Box::new(self.location.geoname_id) as Box<dyn Any + Send + Sync>);
        location_map.insert("capital".to_string(), Box::new(self.location.capital));
        location_map.insert("country_flag".to_string(), Box::new(self.location.country_flag));
        location_map.insert("country_flag_emoji".to_string(), Box::new(self.location.country_flag_emoji));
        location_map.insert("country_flag_emoji_unicode".to_string(), Box::new(self.location.country_flag_emoji_unicode));
        location_map.insert("calling_code".to_string(), Box::new(self.location.calling_code));
        location_map.insert("is_eu".to_string(), Box::new(self.location.is_eu));

        // Handle languages
        let languages_vec: Vec<HashMap<String, String>> = self.location.languages
            .into_iter()
            .map(|lang| {
                let mut lang_map = HashMap::new();
                lang_map.insert("code".to_string(), lang.code);
                lang_map.insert("name".to_string(), lang.name);
                lang_map.insert("native".to_string(), lang.native);
                lang_map
            })
            .collect();
        location_map.insert("languages".to_string(), Box::new(languages_vec));

        response = response.set_output_field("location", location_map);

        response
    }
}