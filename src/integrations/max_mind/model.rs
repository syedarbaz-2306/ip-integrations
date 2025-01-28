use serde::{Deserialize, Serialize};

use crate::integrations::{action_response::{self, ActionResponse}, into_action_response::IntoActionResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct InsightsResponse {
    pub city: City,
    pub continent: Continent,
    pub country: Country,
    pub location: Location,
    pub maxmind: Maxmind,
    pub postal: Postal,
    pub registered_country: Country,
    pub subdivisions: Vec<Subdivision>,
    pub traits: Traits,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    pub confidence: Option<u8>,
    pub geoname_id: u64,
    pub names: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Continent {
    pub code: String,
    pub geoname_id: u64,
    pub names: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub confidence: Option<u8>,
    pub iso_code: String,
    pub geoname_id: u64,
    pub names: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub accuracy_radius: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Maxmind {
    pub queries_remaining: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Postal {
    pub confidence: Option<u8>,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subdivision {
    pub confidence: Option<u8>,
    pub iso_code: String,
    pub geoname_id: u64,
    pub names: Names,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Traits {
    pub static_ip_score: Option<f64>,
    pub user_type: Option<String>,
    pub autonomous_system_number: u64,
    pub autonomous_system_organization: String,
    pub connection_type: String,
    pub isp: String,
    pub organization: String,
    pub ip_address: String,
    pub network: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Names {
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<String>,
    #[serde(rename = "ru")]
    pub ru: Option<String>,
    #[serde(rename = "de")]
    pub de: Option<String>,
    #[serde(rename = "en")]
    pub en: Option<String>,
    #[serde(rename = "es")]
    pub es: Option<String>,
    #[serde(rename = "fr")]
    pub fr: Option<String>,
    #[serde(rename = "ja")]
    pub ja: Option<String>,
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<String>,
}

impl IntoActionResponse for InsightsResponse {
    fn into_action_response(self)-> ActionResponse {
        let action_response = ActionResponse::new()
        .set_output_field("city_confidence", self.city.confidence)
        .set_output_field("city_geoname_id", self.city.geoname_id)
        .set_output_field("city_name", self.city.names.en)
        .set_output_field("continent_code", self.continent.code)
        .set_output_field("continent_geoname_id", self.continent.geoname_id)
        .set_output_field("continent_name", self.continent.names.en)
        .set_output_field("country_confidence", self.country.confidence)
        .set_output_field("country_iso_code", self.country.iso_code)
        .set_output_field("country_geoname_id", self.country.geoname_id)
        .set_output_field("country_name", self.country.names.en)
        .set_output_field("accuracy_radius", self.location.accuracy_radius)
        .set_output_field("latitude", self.location.latitude)
        .set_output_field("longitude", self.location.longitude)
        .set_output_field("time_zone", self.location.time_zone)
        .set_output_field("postal_confidence", self.postal.confidence)
        .set_output_field("postal_code", self.postal.code)
        .set_output_field("registered_country_iso_code", self.registered_country.iso_code)
        .set_output_field("registered_country_geoname_id", self.registered_country.geoname_id)
        .set_output_field("registered_country_name", self.registered_country.names.en)
        .set_output_field("subdivisions_confidence", self.subdivisions[0].confidence)
        .set_output_field("subdivisions_iso_code", self.subdivisions[0].iso_code.clone())
        .set_output_field("subdivisions_geoname_id", self.subdivisions[0].geoname_id)
        .set_output_field("subdivisions_names", self.subdivisions[0].names.en.clone())
        .set_output_field("static_ip_score", self.traits.static_ip_score)
        .set_output_field("user_type", self.traits.user_type)
        .set_output_field("autonomous_system_number", self.traits.autonomous_system_number)
        .set_output_field("autonomous_system_organization", self.traits.autonomous_system_organization)
        .set_output_field("connection_type", self.traits.connection_type)
        .set_output_field("isp", self.traits.isp)
        .set_output_field("organization", self.traits.organization)
        .set_output_field("ip_address", self.traits.ip_address)
        .set_output_field("network", self.traits.network);
    action_response
    }
}