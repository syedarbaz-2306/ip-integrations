use std::{fs::File, io::{BufReader, Read}};
use reqwest::Method;
use sha2::{Sha256, Digest};

use crate::integrations::{make_reqwest::{make_request, RequestConfig}, vt_file::types::FileScanResponse};

pub async fn vt_get_file_report(file_path:String, api_key:String) {
    
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut sha256_hasher = Sha256::new();

    for chunk in reader.bytes() {
        let byte = match chunk {
            Ok(byte) => byte,
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                return;
            }
        };
        sha256_hasher.update([byte]);
    }

    // Finalize the hash
    let sha256_result = sha256_hasher.finalize();

    // Convert the hash to a hexadecimal string
    let sha256_hex = hex::encode(sha256_result);

    // Print the SHA-256 hash
    println!("SHA-256: {}", sha256_hex);



     let url = format!("https://www.virustotal.com/api/v3/files/{}", sha256_hex);

    let config = RequestConfig::new(url, Method::GET).headers(vec![
        ("x-apikey", api_key.clone()),
        ("accept", "application/json".to_string()),
    ]);

    let res = make_request(config).await;

    match res {
        Ok(Some(json))=>{
            let response = serde_json::from_value::<FileScanResponse>(json)
            .map_err(|e| format!("Failed to parse response JSON: {}", e)).unwrap();
            print!("{:?}",response.data.into_action_response());
            
        }
        Ok(None)=>print!("succuess"),
        Err(e)=>print!("{}",e)
    }
}