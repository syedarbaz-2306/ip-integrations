use serde::Deserialize;
use crate::integrations::action_response::ActionResponse;

#[derive(Debug, Deserialize)]
pub struct GeoIPResponseCountry {
    pub continent: Continent,
    pub country: Country,
    pub maxmind: Maxmind,
    pub registered_country: Country,
    pub traits: Trait,
}

#[derive(Debug, Deserialize)]
pub struct Continent {
    pub code: String,
    pub geoname_id: u32,
    pub names: Names,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub iso_code: String,
    pub geoname_id: u32,
    pub names: Names,
}

#[derive(Debug, Deserialize)]
pub struct Maxmind {
    queries_remaining: u32,
}

#[derive(Debug, Deserialize)]
pub struct Trait {
    pub ip_address: String,
    pub network: String,
}

#[derive(Debug, Deserialize)]
pub struct Names {
    pub ja: Option<String>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<String>,
    pub ru: Option<String>,
    #[serde(rename="zh-CN")]
    pub zh_cn: Option<String>,
    pub de: Option<String>,
    pub en: Option<String>,
    pub es: Option<String>,
    pub fr: Option<String>,
}

impl GeoIPResponseCountry {
    pub fn into_action_response(self)-> ActionResponse {
        let action_response =  ActionResponse::new()
        .set_output_field("continent_code", self.continent.code)
        .set_output_field("continent_geoname_id", self.continent.geoname_id)
        .set_output_field("continent_names_ja", self.continent.names.ja)
        .set_output_field("continent_names_pt_br", self.continent.names.pt_br)
        .set_output_field("continent_names_ru", self.continent.names.ru)
        .set_output_field("continent_names_zh_cn", self.continent.names.zh_cn)
        .set_output_field("continent_names_de", self.continent.names.de)
        .set_output_field("continent_names_en", self.continent.names.en)
        .set_output_field("continent_names_es", self.continent.names.es)
        .set_output_field("continent_names_fr", self.continent.names.fr)
        .set_output_field("country_code", self.country.iso_code)
        .set_output_field("country_geoname_id", self.country.geoname_id)
        .set_output_field("country_names_ja", self.country.names.ja)
        .set_output_field("country_names_pt_br", self.country.names.pt_br)
        .set_output_field("country_names_ru", self.country.names.ru)
        .set_output_field("country_names_zh_cn", self.country.names.zh_cn)
        .set_output_field("country_names_de", self.country.names.de)
        .set_output_field("country_names_en", self.country.names.en)
        .set_output_field("country_names_es", self.country.names.es)
        .set_output_field("country_names_fr", self.country.names.fr)
        .set_output_field("registered_country_iso_code", self.registered_country.iso_code)
        .set_output_field("registered_country_geoname_id", self.registered_country.geoname_id)
        .set_output_field("registered_country_names_ja", self.registered_country.names.ja)
        .set_output_field("registered_country_names_pt_br", self.registered_country.names.pt_br)
        .set_output_field("registered_country_names_ru", self.registered_country.names.ru)
        .set_output_field("registered_country_names_zh_cn", self.registered_country.names.zh_cn)
        .set_output_field("registered_country_names_de", self.registered_country.names.de)
        .set_output_field("registered_country_names_en", self.registered_country.names.en)
        .set_output_field("registered_country_names_es", self.registered_country.names.es)
        .set_output_field("registered_country_names_fr", self.registered_country.names.fr)
        .set_output_field("ip_address", self.traits.ip_address)
        .set_output_field("network", self.traits.network)
        .set_output_field("queries_remaining", self.maxmind.queries_remaining);
        action_response
    }
}

#[derive(Debug, Deserialize)]
pub struct GeoIPResponseCity {
    pub city:City,
    pub continent:Continent,
    pub country:Country,
    pub location:Location,
    pub maxmind:Maxmind,
    pub postal:Postal,
    pub registered_country:Country,
    pub subdivisions:Vec<Country>,
    pub traits:Traits,
}

#[derive(Debug, Deserialize)]
pub struct City {
    pub geoname_id: u32,
    pub names:Names,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub accuracy_radius:u32,
    pub latitude:f32,
    pub longitude:f32,
    pub time_zone:String,
}

#[derive(Debug, Deserialize)]
pub struct Postal {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct Traits {
    pub autonomous_system_number:u32,
    pub autonomous_system_organization:String,
    pub connection_type:String,
    pub isp:String,
    pub organization:String,
    pub ip_address:String,
    pub network:String,
}

impl GeoIPResponseCity {
    pub fn into_action_response(self)-> ActionResponse {
        let action_response = ActionResponse::new()
        .set_output_field("city_geoname_id", self.city.geoname_id)
        .set_output_field("city_name", self.city.names.en)
        .set_output_field("continent_code", self.continent.code)
        .set_output_field("continent_geoname_id", self.city.geoname_id)
        .set_output_field("country_iso_code", self.country.iso_code)
        .set_output_field("country_geoname_id", self.country.geoname_id)
        .set_output_field("accuracy_radius", self.location.accuracy_radius)
        .set_output_field("latitude", self.location.latitude)
        .set_output_field("longitude", self.location.longitude)
        .set_output_field("time_zone", self.location.time_zone)
        .set_output_field("queries_remaining", self.maxmind.queries_remaining)
        .set_output_field("postal_code", self.postal.code)
        .set_output_field("registered_country_iso_code", self.registered_country.iso_code)
        .set_output_field("registered_country_geoname_id", self.registered_country.geoname_id)
        .set_output_field("subdivisions_iso_code", self.subdivisions[0].iso_code.clone())
        .set_output_field("subdivisions_geoname_id", self.subdivisions[0].geoname_id)
        .set_output_field("subdivisions_name", self.subdivisions[0].names.en.clone())
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