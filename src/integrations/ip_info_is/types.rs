use serde::{Deserialize, Serialize};
use crate::integrations::action_response::ActionResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpinfoIs {
    pub area_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<Country>,
    pub domain: Option<String>,
    pub idd_code: Option<String>,
    pub ip: Option<String>,
    pub isp: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub net_speed: Option<String>,
    pub region: Option<String>,
    pub time_zone: Option<String>,
    pub weather_station_code: Option<String>,
    pub weather_station_name: Option<String>,
    pub zip_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub long_name: Option<String>,
    pub short_name: Option<String>,
}


impl IpinfoIs {
    pub fn into_action_response(self) -> ActionResponse {
        let mut action_response = ActionResponse::new();

        // Append all fields to the ActionResponse
        if let Some(area_code) = self.area_code {
            action_response = action_response.set_output_field("area_code", area_code);
        }
        if let Some(city) = self.city {
            action_response = action_response.set_output_field("city", city);
        }
        if let Some(country) = self.country {
            if let Some(long_name) = country.long_name {
                action_response = action_response.set_output_field("country_long_name", long_name);
            }
            if let Some(short_name) = country.short_name {
                action_response = action_response.set_output_field("country_short_name", short_name);
            }
        }
        if let Some(domain) = self.domain {
            action_response = action_response.set_output_field("domain", domain);
        }
        if let Some(idd_code) = self.idd_code {
            action_response = action_response.set_output_field("idd_code", idd_code);
        }
        if let Some(ip) = self.ip {
            action_response = action_response.set_output_field("ip", ip);
        }
        if let Some(isp) = self.isp {
            action_response = action_response.set_output_field("isp", isp);
        }
        if let Some(latitude) = self.latitude {
            action_response = action_response.set_output_field("latitude", latitude);
        }
        if let Some(longitude) = self.longitude {
            action_response = action_response.set_output_field("longitude", longitude);
        }
        if let Some(net_speed) = self.net_speed {
            action_response = action_response.set_output_field("net_speed", net_speed);
        }
        if let Some(region) = self.region {
            action_response = action_response.set_output_field("region", region);
        }
        if let Some(time_zone) = self.time_zone {
            action_response = action_response.set_output_field("time_zone", time_zone);
        }
        if let Some(weather_station_code) = self.weather_station_code {
            action_response = action_response.set_output_field("weather_station_code", weather_station_code);
        }
        if let Some(weather_station_name) = self.weather_station_name {
            action_response = action_response.set_output_field("weather_station_name", weather_station_name);
        }
        if let Some(zip_code) = self.zip_code {
            action_response = action_response.set_output_field("zip_code", zip_code);
        }

        action_response
    }
}