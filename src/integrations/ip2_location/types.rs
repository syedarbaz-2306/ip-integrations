use serde::{Deserialize, Serialize};

use crate::integrations::action_response::ActionResponse;


#[derive(Serialize, Deserialize, Debug)]
pub struct Ip2Location {
    ip: String,
    country_code: String,
    country_name: String,
    region_name: String,
    city_name: String,
    latitude: f64,
    longitude: f64,
    zip_code: String,
    time_zone: String,
    asn: String,
    #[serde(rename = "as")]
    as_name: String,
    is_proxy: bool,
}

impl Ip2Location {
    pub fn into_action_response(self) -> ActionResponse {
        let mut action_response = ActionResponse::new();
        action_response = action_response
            .set_output_field("ip", self.ip)
            .set_output_field("country_code", self.country_code)
            .set_output_field("country_name", self.country_name)
            .set_output_field("region_name", self.region_name)
            .set_output_field("city_name", self.city_name)
            .set_output_field("latitude", self.latitude)
            .set_output_field("longitude", self.longitude)
            .set_output_field("zip_code", self.zip_code)
            .set_output_field("time_zone", self.time_zone)
            .set_output_field("asn", self.asn)
            .set_output_field("as", self.as_name)
            .set_output_field("is_proxy", self.is_proxy);

        action_response
    }
}