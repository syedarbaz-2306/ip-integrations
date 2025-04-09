use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::integrations::action_response::ActionResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainResponse {
    pub data: DomainData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainData {
    pub id: String,
    #[serde(rename = "type")]
    pub domain_type: String,
    pub links: DomainLinks,
    pub attributes: DomainAttributes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainLinks {
    #[serde(rename = "self")]
    pub self_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainAttributes {
    pub registrar: Option<String>,
    #[serde(rename = "total_votes")]
    pub total_votes: TotalVotes,
    #[serde(rename = "last_dns_records")]
    pub last_dns_records: Vec<DnsRecord>,
    pub jarm: Option<String>,
    #[serde(rename = "last_dns_records_date")]
    pub last_dns_records_date: Option<u64>,
    #[serde(rename = "last_modification_date")]
    pub last_modification_date: Option<u64>,
    #[serde(rename = "whois_date")]
    pub whois_date: Option<u64>,
    #[serde(rename = "creation_date")]
    pub creation_date: Option<u64>,
    #[serde(rename = "popularity_ranks")]
    pub popularity_ranks: Option<HashMap<String, PopularityRank>>,
    #[serde(rename = "last_https_certificate")]
    pub last_https_certificate: Option<HttpsCertificate>,
    pub tags: Vec<String>,
    #[serde(rename = "last_analysis_stats")]
    pub last_analysis_stats: AnalysisStats,
    #[serde(rename = "last_analysis_results")]
    pub last_analysis_results: HashMap<String, AnalysisResult>,
    #[serde(rename = "last_analysis_date")]
    pub last_analysis_date: Option<u64>,
    pub tld: String,
    #[serde(rename = "last_update_date")]
    pub last_update_date: Option<u64>,
    pub whois: Option<String>,
    pub categories: Option<HashMap<String, String>>,
    pub reputation: Option<i32>,
    #[serde(rename = "last_https_certificate_date")]
    pub last_https_certificate_date: Option<u64>,
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalVotes {
    pub harmless: u32,
    pub malicious: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsRecord {
    #[serde(rename = "type")]
    pub record_type: String,
    pub ttl: u32,
    pub value: String,
    pub rname: Option<String>,
    pub serial: Option<u64>,
    pub refresh: Option<u64>,
    pub retry: Option<u64>,
    pub expire: Option<u64>,
    pub minimum: Option<u64>,
    pub flag: Option<u32>,
    pub tag: Option<String>,
    pub priority: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PopularityRank {
    pub rank: u32,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpsCertificate {
    pub cert_signature: CertSignature,
    pub extensions: CertificateExtensions,
    pub validity: Validity,
    pub size: u32,
    pub version: String,
    pub public_key: PublicKey,
    pub thumbprint_sha256: String,
    pub thumbprint: String,
    pub serial_number: String,
    pub issuer: Issuer,
    pub subject: Subject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertSignature {
    #[serde(rename = "signature_algorithm")]
    pub signature_algorithm: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateExtensions {
    #[serde(rename = "key_usage")]
    pub key_usage: Vec<String>,
    #[serde(rename = "extended_key_usage")]
    pub extended_key_usage: Vec<String>,
    #[serde(rename = "CA")]
    pub ca: bool,
    #[serde(rename = "subject_key_identifier")]
    pub subject_key_identifier: String,
    #[serde(rename = "authority_key_identifier")]
    pub authority_key_identifier: AuthorityKeyIdentifier,
    #[serde(rename = "ca_information_access")]
    pub ca_information_access: CaInformationAccess,
    #[serde(rename = "subject_alternative_name")]
    pub subject_alternative_name: Vec<String>,
    #[serde(rename = "certificate_policies")]
    pub certificate_policies: Vec<String>,
    #[serde(rename = "crl_distribution_points")]
    pub crl_distribution_points: Vec<String>,
    #[serde(rename = "1.3.6.1.4.1.11129.2.4.2")]
    pub oid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorityKeyIdentifier {
    pub keyid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaInformationAccess {
    #[serde(rename = "OCSP")]
    pub ocsp: String,
    #[serde(rename = "CA Issuers")]
    pub ca_issuers: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Validity {
    #[serde(rename = "not_after")]
    pub not_after: String,
    #[serde(rename = "not_before")]
    pub not_before: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub algorithm: String,
    pub ec: Option<EcKey>,
    pub rsa: Option<RsaKey>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EcKey {
    pub oid: String,
    #[serde(rename = "pub")]
    pub _pub: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RsaKey {
    pub modulus: String,
    pub exponent: String,
    #[serde(rename = "key_size")]
    pub key_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issuer {
    #[serde(rename = "C")]
    pub c: String,
    #[serde(rename = "O")]
    pub o: String,
    #[serde(rename = "CN")]
    pub cn: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    #[serde(rename = "CN")]
    pub cn: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisStats {
    pub malicious: u32,
    pub suspicious: u32,
    pub undetected: u32,
    pub harmless: u32,
    pub timeout: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisResult {
    pub method: String,
    #[serde(rename = "engine_name")]
    pub engine_name: String,
    pub category: String,
    pub result: String,
}


impl DomainData {
    pub fn into_action_response(self) -> ActionResponse {
        let atr = self.attributes;

        let mut popular_ranks: Vec<String> = Vec::new();
        if let Some(ranks) = atr.popularity_ranks {
            for (key, value) in ranks {
                popular_ranks.push(format!("{} rank: {}", key, value.rank));
            }
        }

        let mut categories_vec: Vec<String> = Vec::new();
        if let Some(categories) = atr.categories {
            for (key, value) in categories {
                categories_vec.push(format!("{}: {}", key, value));
            }
        }

        let mut analysis_results_vec: Vec<String> = Vec::new();
        for (key, value) in atr.last_analysis_results {
            analysis_results_vec.push(format!("{} result: {}", key, value.result));
        }

        let mut dns_records_vec: Vec<String> = Vec::new();
        for record in atr.last_dns_records {
            dns_records_vec.push(format!(
                "Type: {}, TTL: {}, Value: {}",
                record.record_type, record.ttl, record.value
            ));
        }

        let mut action_response = ActionResponse::new();
        action_response = action_response
            .set_output_field("id", self.id)
            .set_output_field("type", self.domain_type)
            .set_output_field("self", self.links.self_url);

        action_response = action_response
            .set_output_field("last_https_certificate_date", atr.last_https_certificate_date)
            .set_output_field("last_modification_date", atr.last_modification_date)
            .set_output_field("last_update_date", atr.last_update_date)
            .set_output_field("last_dns_records_date", atr.last_dns_records_date)
            .set_output_field("popularity_ranks", popular_ranks.join(",\n"))
            .set_output_field("reputation", atr.reputation)
            .set_output_field("categories", categories_vec.join(",\n"))
            .set_output_field("last_analysis_date", atr.last_analysis_date)
            .set_output_field("last_analysis_stats_malicious", atr.last_analysis_stats.malicious)
            .set_output_field("last_analysis_stats_suspicious", atr.last_analysis_stats.suspicious)
            .set_output_field("last_analysis_stats_undetected", atr.last_analysis_stats.undetected)
            .set_output_field("last_analysis_stats_harmless", atr.last_analysis_stats.harmless)
            .set_output_field("last_analysis_stats_timeout", atr.last_analysis_stats.timeout)
            .set_output_field("whois", atr.whois.unwrap_or_default())
            .set_output_field("last_analysis_results", analysis_results_vec.join(",\n"))
            .set_output_field("tld", atr.tld)
            .set_output_field("jarm", atr.jarm.unwrap_or_default())
            .set_output_field("total_votes_harmless", atr.total_votes.harmless)
            .set_output_field("total_votes_malicious", atr.total_votes.malicious)
            .set_output_field("registrar", atr.registrar.unwrap_or_default())
            .set_output_field("creation_date", atr.creation_date)
            .set_output_field("expiration_date", atr.expiration_date)
            .set_output_field("last_dns_records", dns_records_vec.join(",\n"))
            .set_output_field("tags", atr.tags.join(", "));

        if let Some(cert) = atr.last_https_certificate {
            action_response = action_response
                .set_output_field("cert_sign_algo", cert.cert_signature.signature_algorithm)
                .set_output_field("cert_signature", cert.cert_signature.signature)
                .set_output_field("cert_extensions_key_usage", cert.extensions.key_usage.join(",\n"))
                .set_output_field("cert_extensions_extended_key_usage", cert.extensions.extended_key_usage.join(",\n"))
                .set_output_field("cert_extensions_ca", cert.extensions.ca)
                .set_output_field("cert_extensions_subject_key_identifier", cert.extensions.subject_key_identifier)
                .set_output_field("cert_extensions_authority_key_identifier", cert.extensions.authority_key_identifier.keyid)
                .set_output_field("cert_extensions_ca_information_access_ocsp", cert.extensions.ca_information_access.ocsp)
                .set_output_field("cert_extensions_ca_information_access_ca_issuers", cert.extensions.ca_information_access.ca_issuers)
                .set_output_field("cert_validity_not_after", cert.validity.not_after)
                .set_output_field("cert_validity_not_before", cert.validity.not_before)
                .set_output_field("cert_size", cert.size)
                .set_output_field("cert_version", cert.version)
                .set_output_field("cert_thumbprint_sha256", cert.thumbprint_sha256)
                .set_output_field("cert_thumbprint", cert.thumbprint)
                .set_output_field("cert_serial_number", cert.serial_number)
                .set_output_field("cert_issuer_C", cert.issuer.c)
                .set_output_field("cert_issuer_O", cert.issuer.o)
                .set_output_field("cert_issuer_CN", cert.issuer.cn)
                .set_output_field("cert_subject_CN", cert.subject.cn);

            if let Some(ec_key) = cert.public_key.ec {
                action_response = action_response
                    .set_output_field("cert_public_key_algorithm", "EC")
                    .set_output_field("cert_public_key_oid", ec_key.oid)
                    .set_output_field("cert_public_key_pub", ec_key._pub);
            } else if let Some(rsa_key) = cert.public_key.rsa {
                action_response = action_response
                    .set_output_field("cert_public_key_algorithm", "RSA")
                    .set_output_field("cert_public_key_modulus", rsa_key.modulus)
                    .set_output_field("cert_public_key_exponent", rsa_key.exponent)
                    .set_output_field("cert_public_key_size", rsa_key.key_size);
            }
        }
        action_response
    }
}
