use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::integrations::action_response::ActionResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct FileScanResponse {
    pub data: VirusTotalData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VirusTotalData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub links: Links,
    pub attributes: Attributes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attributes {
    #[serde(rename = "authentihash")]
    pub authentihash: Option<String>,
    pub tlsh: Option<String>,
    #[serde(rename = "pe_info")]
    pub pe_info: Option<PeInfo>,
    #[serde(rename = "first_submission_date")]
    pub first_submission_date: Option<i64>,
    #[serde(rename = "last_analysis_stats")]
    pub last_analysis_stats: LastAnalysisStats,
    #[serde(rename = "sigma_analysis_results")]
    pub sigma_analysis_results: Option<Vec<SigmaAnalysisResult>>,
    pub trid: Option<Vec<Trid>>,
    pub size: Option<u64>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "meaningful_name")]
    pub meaningful_name: Option<String>,
    #[serde(rename = "names")]
    pub names: Option<Vec<String>>,
    pub sha1: Option<String>,
    pub reputation: Option<i32>,
    pub ssdeep: Option<String>,
    #[serde(rename = "last_submission_date")]
    pub last_submission_date: Option<i64>,
    #[serde(rename = "type_extension")]
    pub type_extension: Option<String>,
    #[serde(rename = "unique_sources")]
    pub unique_sources: Option<u64>,
    #[serde(rename = "signature_info")]
    pub signature_info: Option<SignatureInfo>,
    #[serde(rename = "last_analysis_results")]
    pub last_analysis_results: LastAnalysisResults,
    #[serde(rename = "type_tags")]
    pub type_tags: Option<Vec<String>>,
    #[serde(rename = "last_analysis_date")]
    pub last_analysis_date: Option<i64>,
    #[serde(rename = "vhash")]
    pub vhash: Option<String>,
    #[serde(rename = "magic")]
    pub magic: Option<String>,
    #[serde(rename = "detectiteasy")]
    pub detectiteasy: Option<DetectItEasy>,
    #[serde(rename = "filecondis")]
    pub filecondis: Option<FileCondis>,
    #[serde(rename = "last_modification_date")]
    pub last_modification_date: Option<i64>,
    #[serde(rename = "total_votes")]
    pub total_votes: Option<TotalVotes>,
    #[serde(rename = "creation_date")]
    pub creation_date: Option<i64>,
    #[serde(rename = "md5")]
    pub md5: Option<String>,
    #[serde(rename = "sha256")]
    pub sha256: Option<String>,
    #[serde(rename = "type_tag")]
    pub type_tag: Option<String>,
    #[serde(rename = "magika")]
    pub magika: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PeInfo {
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    #[serde(rename = "imphash")]
    pub imphash: Option<String>,
    #[serde(rename = "machine_type")]
    pub machine_type: Option<u32>,
    #[serde(rename = "entry_point")]
    pub entry_point: Option<u32>,
    #[serde(rename = "resource_details")]
    pub resource_details: Option<Vec<ResourceDetail>>,
    #[serde(rename = "resource_langs")]
    pub resource_langs: Option<HashMap<String, u32>>,
    #[serde(rename = "resource_types")]
    pub resource_types: Option<HashMap<String, u32>>,
    #[serde(rename = "overlay")]
    pub overlay: Option<Overlay>,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<Section>>,
    #[serde(rename = "compiler_product_versions")]
    pub compiler_product_versions: Option<Vec<String>>,
    #[serde(rename = "rich_pe_header_hash")]
    pub rich_pe_header_hash: Option<String>,
    #[serde(rename = "import_list")]
    pub import_list: Option<Vec<ImportLibrary>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceDetail {
    pub lang: String,
    pub chi2: f64,
    #[serde(rename = "filetype")]
    pub filetype: String,
    pub entropy: f64,
    #[serde(rename = "sha256")]
    pub sha256: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Overlay {
    pub chi2: f64,
    #[serde(rename = "filetype")]
    pub filetype: String,
    pub entropy: f64,
    pub offset: u64,
    pub md5: String,
    pub size: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub name: String,
    pub chi2: f64,
    #[serde(rename = "virtual_address")]
    pub virtual_address: u64,
    pub entropy: f64,
    #[serde(rename = "raw_size")]
    pub raw_size: u64,
    pub flags: String,
    #[serde(rename = "virtual_size")]
    pub virtual_size: u64,
    pub md5: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportLibrary {
    #[serde(rename = "library_name")]
    pub library_name: String,
    #[serde(rename = "imported_functions")]
    pub imported_functions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SigmaAnalysisResult {
    #[serde(rename = "rule_level")]
    pub rule_level: String,
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    #[serde(rename = "rule_source")]
    pub rule_source: String,
    #[serde(rename = "rule_title")]
    pub rule_title: String,
    #[serde(rename = "rule_description")]
    pub rule_description: String,
    #[serde(rename = "rule_author")]
    pub rule_author: String,
    #[serde(rename = "match_context")]
    pub match_context: Vec<MatchContext>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MatchContext {
    pub values: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trid {
    #[serde(rename = "file_type")]
    pub file_type: String,
    pub probability: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastAnalysisStats {
    pub malicious: u32,
    pub suspicious: u32,
    pub undetected: u32,
    pub harmless: u32,
    #[serde(rename = "timeout")]
    pub timeout: u32,
    #[serde(rename = "confirmed-timeout")]
    pub confirmed_timeout: u32,
    pub failure: u32,
    #[serde(rename = "type-unsupported")]
    pub type_unsupported: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureInfo {
    pub product: Option<String>,
    pub verified: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "file version")]
    pub file_version: Option<String>,
    #[serde(rename = "signing date")]
    pub signing_date: Option<String>,
    pub x509: Option<Vec<X509>>,
    #[serde(rename = "original name")]
    pub original_name: Option<String>,
    pub signers: Option<String>,
    #[serde(rename = "counter signers details")]
    pub counter_signers_details: Option<Vec<CounterSigner>>,
    #[serde(rename = "counter signers")]
    pub counter_signers: Option<String>,
    #[serde(rename = "internal name")]
    pub internal_name: Option<String>,
    pub copyright: Option<String>,
    #[serde(rename = "signers details")]
    pub signers_details: Option<Vec<SignerDetail>>,
    pub pkcs7: Option<Pkcs7>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct X509 {
    #[serde(rename = "valid usage")]
    pub valid_usage: Option<String>,
    #[serde(rename = "thumbprint_sha256")]
    pub thumbprint_sha256: String,
    pub name: String,
    pub algorithm: String,
    #[serde(rename = "thumbprint_md5")]
    pub thumbprint_md5: String,
    #[serde(rename = "valid from")]
    pub valid_from: String,
    #[serde(rename = "valid to")]
    pub valid_to: String,
    #[serde(rename = "serial number")]
    pub serial_number: String,
    #[serde(rename = "cert issuer")]
    pub cert_issuer: String,
    pub thumbprint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CounterSigner {
    pub status: String,
    #[serde(rename = "valid usage")]
    pub valid_usage: Option<String>,
    pub name: String,
    pub algorithm: String,
    #[serde(rename = "valid from")]
    pub valid_from: String,
    #[serde(rename = "valid to")]
    pub valid_to: String,
    #[serde(rename = "serial number")]
    pub serial_number: String,
    #[serde(rename = "cert issuer")]
    pub cert_issuer: String,
    pub thumbprint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignerDetail {
    pub status: String,
    #[serde(rename = "valid usage")]
    pub valid_usage: Option<String>,
    pub name: String,
    pub algorithm: String,
    #[serde(rename = "valid from")]
    pub valid_from: String,
    #[serde(rename = "valid to")]
    pub valid_to: String,
    #[serde(rename = "serial number")]
    pub serial_number: String,
    #[serde(rename = "cert issuer")]
    pub cert_issuer: String,
    pub thumbprint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pkcs7 {
    pub opusinfo: Vec<OpusInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpusInfo {
    #[serde(rename = "moreInfo")]
    pub more_info: String,
    #[serde(rename = "programName")]
    pub program_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastAnalysisResults {
    #[serde(flatten)]
    pub engines: HashMap<String, EngineResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EngineResult {
    pub method: String,
    #[serde(rename = "engine_name")]
    pub engine_name: String,
    #[serde(rename = "engine_version")]
    pub engine_version: String,
    #[serde(rename = "engine_update")]
    pub engine_update: String,
    pub category: String,
    pub result: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DetectItEasy {
    #[serde(rename = "filetype")]
    pub filetype: String,
    pub values: Vec<DetectorValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DetectorValue {
    pub info: Option<String>,
    pub version: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileCondis {
    pub dhash: String,
    #[serde(rename = "raw_md5")]
    pub raw_md5: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TotalVotes {
    pub harmless: u32,
    pub malicious: u32,
}

impl VirusTotalData {
    pub fn into_action_response(self) -> ActionResponse {
        let mut action_response = ActionResponse::new()
            .set_output_field("id", self.id);

        // Set other fields
        if let Some(ref authentihash) = self.attributes.authentihash {
            action_response = action_response.set_output_field("authentihash", authentihash.clone());
        }

        if let Some(ref tlsh) = self.attributes.tlsh {
            action_response = action_response.set_output_field("tlsh", tlsh.clone());
        }

        if let Some(ref pe_info) = self.attributes.pe_info {
            if let Some(ref imphash) = pe_info.imphash {
                action_response = action_response.set_output_field("imphash", imphash.clone());
            }
        }

        if let Some(first_submission_date) = self.attributes.first_submission_date {
            action_response = action_response.set_output_field("first_submission_date", first_submission_date.to_string());
        }

        // Set last_analysis_stats fields
        let last_analysis_stats = &self.attributes.last_analysis_stats;
        action_response = action_response
            .set_output_field("malicious", last_analysis_stats.malicious.to_string())
            .set_output_field("suspicious", last_analysis_stats.suspicious.to_string())
            .set_output_field("undetected", last_analysis_stats.undetected.to_string())
            .set_output_field("harmless", last_analysis_stats.harmless.to_string())
            .set_output_field("timeout", last_analysis_stats.timeout.to_string())
            .set_output_field("confirmed_timeout", last_analysis_stats.confirmed_timeout.to_string())
            .set_output_field("failure", last_analysis_stats.failure.to_string())
            .set_output_field("type_unsupported", last_analysis_stats.type_unsupported.to_string());

        // Set other fields
        if let Some(ref meaningful_name) = self.attributes.meaningful_name {
            action_response = action_response.set_output_field("meaningful_name", meaningful_name.clone());
        }

        if let Some(ref sha1) = self.attributes.sha1 {
            action_response = action_response.set_output_field("sha1", sha1.clone());
        }

        if let Some(reputation) = self.attributes.reputation {
            action_response = action_response.set_output_field("reputation", reputation.to_string());
        }

        if let Some(ref ssdeep) = self.attributes.ssdeep {
            action_response = action_response.set_output_field("ssdeep", ssdeep.clone());
        }

        if let Some(last_submission_date) = self.attributes.last_submission_date {
            action_response = action_response.set_output_field("last_submission_date", last_submission_date.to_string());
        }

        if let Some(ref type_extension) = self.attributes.type_extension {
            action_response = action_response.set_output_field("type_extension", type_extension.clone());
        }

        if let Some(unique_sources) = self.attributes.unique_sources {
            action_response = action_response.set_output_field("unique_sources", unique_sources.to_string());
        }

        if let Some(ref last_analysis_date) = self.attributes.last_analysis_date {
            action_response = action_response.set_output_field("last_analysis_date", last_analysis_date.to_string());
        }

        if let Some(ref vhash) = self.attributes.vhash {
            action_response = action_response.set_output_field("vhash", vhash.clone());
        }

        if let Some(ref magic) = self.attributes.magic {
            action_response = action_response.set_output_field("magic", magic.clone());
        }

        if let Some(ref md5) = self.attributes.md5 {
            action_response = action_response.set_output_field("md5", md5.clone());
        }

        if let Some(ref sha256) = self.attributes.sha256 {
            action_response = action_response.set_output_field("sha256", sha256.clone());
        }

        if let Some(ref type_tag) = self.attributes.type_tag {
            action_response = action_response.set_output_field("type_tag", type_tag.clone());
        }

        if let Some(ref magika) = self.attributes.magika {
            action_response = action_response.set_output_field("magika", magika.clone());
        }

        action_response
    }
}