use serde::{Deserialize, Serialize};

use crate::integrations::action_response::ActionResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct IpinfoIo {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
}

impl IpinfoIo {
    pub fn into_action_response(self)-> ActionResponse {
       let action_response = ActionResponse::new()
       .set_output_field("ip", self.ip)
       .set_output_field("city", self.city)
       .set_output_field("region", self.region)
       .set_output_field("country", self.country)
       .set_output_field("loc", self.loc)
       .set_output_field("isp", self.org)
       .set_output_field("postal", self.postal)
       .set_output_field("timezone", self.timezone);
    action_response
    }
}