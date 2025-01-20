use std::{any::Any, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::integrations::action_response::ActionResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpStack {
    pub city: Option<String>,
    pub connection_type: Option<Value>, // Nullable field
    pub continent_code: Option<String>,
    pub continent_name: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub dma: Option<String>,
    pub ip: Option<String>,
    pub ip_routing_type: Option<Value>, // Nullable field
    pub latitude: Option<f64>,
    pub location: Option<Location>,
    pub longitude: Option<f64>,
    pub msa: Option<String>,
    pub radius: Option<Value>, // Nullable field
    pub region_code: Option<String>,
    pub region_name: Option<String>,
    pub type_: Option<String>, // `type` is a reserved keyword in Rust, so we use `type_`
    pub zip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub calling_code: Option<String>,
    pub capital: Option<String>,
    pub country_flag: Option<String>,
    pub country_flag_emoji: Option<String>,
    pub country_flag_emoji_unicode: Option<String>,
    pub geoname_id: Option<u64>,
    pub is_eu: Option<bool>,
    pub languages: Option<Vec<Language>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub code: Option<String>,
    pub name: Option<String>,
    pub native: Option<String>,
}


impl IpStack {
    pub fn into_action_response(self) -> ActionResponse {
        let mut response = ActionResponse::new();

        // String fields - no need for to_string() since they're already String
        if let Some(city) = self.city {
            response = response.set_output_field("city", city);
        }
        if let Some(continent_code) = self.continent_code {
            response = response.set_output_field("continent_code", continent_code);
        }
        if let Some(continent_name) = self.continent_name {
            response = response.set_output_field("continent_name", continent_name);
        }
        if let Some(country_code) = self.country_code {
            response = response.set_output_field("country_code", country_code);
        }
        if let Some(country_name) = self.country_name {
            response = response.set_output_field("country_name", country_name);
        }
        if let Some(dma) = self.dma {
            response = response.set_output_field("dma", dma);
        }
        if let Some(ip) = self.ip {
            response = response.set_output_field("ip", ip);
        }
        if let Some(msa) = self.msa {
            response = response.set_output_field("msa", msa);
        }
        if let Some(region_code) = self.region_code {
            response = response.set_output_field("region_code", region_code);
        }
        if let Some(region_name) = self.region_name {
            response = response.set_output_field("region_name", region_name);
        }
        if let Some(type_) = self.type_ {
            response = response.set_output_field("type", type_);
        }
        if let Some(zip) = self.zip {
            response = response.set_output_field("zip", zip);
        }

        // Numeric fields (f64)
        if let Some(latitude) = self.latitude {
            response = response.set_output_field("latitude", latitude);
        }
        if let Some(longitude) = self.longitude {
            response = response.set_output_field("longitude", longitude);
        }

        // Value fields (serde_json::Value)
        if let Some(connection_type) = self.connection_type {
            response = response.set_output_field("connection_type", connection_type);
        }
        if let Some(ip_routing_type) = self.ip_routing_type {
            response = response.set_output_field("ip_routing_type", ip_routing_type);
        }
        if let Some(radius) = self.radius {
            response = response.set_output_field("radius", radius);
        }

        // Location struct
        if let Some(location) = self.location {
            let mut location_map: HashMap<String, Box<dyn Any + Send + Sync>> = HashMap::new();

            // String fields in Location
            if let Some(calling_code) = location.calling_code {
                location_map.insert("calling_code".to_string(), Box::new(calling_code));
            }
            if let Some(capital) = location.capital {
                location_map.insert("capital".to_string(), Box::new(capital));
            }
            if let Some(country_flag) = location.country_flag {
                location_map.insert("country_flag".to_string(), Box::new(country_flag));
            }
            if let Some(country_flag_emoji) = location.country_flag_emoji {
                location_map.insert("country_flag_emoji".to_string(), Box::new(country_flag_emoji));
            }
            if let Some(country_flag_emoji_unicode) = location.country_flag_emoji_unicode {
                location_map.insert("country_flag_emoji_unicode".to_string(), Box::new(country_flag_emoji_unicode));
            }

            // Numeric and boolean fields in Location
            if let Some(geoname_id) = location.geoname_id {
                location_map.insert("geoname_id".to_string(), Box::new(geoname_id));
            }
            if let Some(is_eu) = location.is_eu {
                location_map.insert("is_eu".to_string(), Box::new(is_eu));
            }

            // Languages vector
            if let Some(languages) = location.languages {
                let languages_vec: Vec<HashMap<String, String>> = languages
                    .into_iter()
                    .map(|lang| {
                        let mut lang_map = HashMap::new();
                        if let Some(code) = lang.code {
                            lang_map.insert("code".to_string(), code);
                        }
                        if let Some(name) = lang.name {
                            lang_map.insert("name".to_string(), name);
                        }
                        if let Some(native) = lang.native {
                            lang_map.insert("native".to_string(), native);
                        }
                        lang_map
                    })
                    .collect();
                location_map.insert("languages".to_string(), Box::new(languages_vec));
            }

            response = response.set_output_field("location", location_map);
        }

        response
    }
}

// Helper function to extract values from ActionResponse
