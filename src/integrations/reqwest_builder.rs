use reqwest::{
    Client, Method, 
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    multipart::{Form, Part},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf};
use std::fs::File;
use std::io::Read;
use super::{action_response::ActionResponse, into_action_response::IntoActionResponse};

#[derive(Debug)]
pub enum RequestBody {
    Json(Value),
    FormUrlEncoded(HashMap<String, String>),
    MultipartForm(Vec<(String, MultipartContent)>),
    Text(String),
    Xml(String),
    Binary(Vec<u8>),
    None,
}

#[derive(Debug)]
pub enum MultipartContent {
    File {
        path: PathBuf,
        filename: Option<String>,
        content_type: Option<String>,
    },
    Text {
        content: String,
        content_type: Option<String>,
    },
    Bytes {
        content: Vec<u8>,
        filename: Option<String>,
        content_type: Option<String>,
    },
}

#[derive(Debug)]
pub struct RequestConfig {
    pub url: String,
    pub method: Method,
    pub params: Option<HashMap<String, String>>,
    pub body: RequestBody,
    pub headers: Option<HeaderMap>,
    pub auth: Option<Auth>,
}

#[derive(Debug)]
pub enum Auth {
    Basic {
        username: String,
        password: Option<String>,
    },
    Bearer(String),
}

impl RequestConfig {
    pub fn new<S: Into<String>>(url: S, method: Method) -> Self {
        Self {
            url: url.into(),
            method,
            params: None,
            body: RequestBody::None,
            headers: None,
            auth: None,
        }
    }

    // Method to accept array of tuples for params
    pub fn params<K, V>(mut self, params: &[(K, V)]) -> Self 
    where 
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut map = HashMap::new();
        for (key, value) in params {
            map.insert(key.as_ref().to_string(), value.as_ref().to_string());
        }
        self.params = Some(map);
        self
    }

    // For backward compatibility
    pub fn with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params = Some(params);
        self
    }

    // Method to accept array of tuples for headers
    pub fn headers<K, V>(mut self, headers: &[(K, V)]) -> Self 
    where 
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_bytes(key.as_ref().as_bytes()),
                HeaderValue::from_str(value.as_ref())
            ) {
                header_map.insert(name, val);
            }
        }
        self.headers = Some(header_map);
        self
    }

    // For backward compatibility
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn json_body<T: serde::Serialize>(mut self, body: T) -> Self {
        if let Ok(json) = serde_json::to_value(body) {
            self.body = RequestBody::Json(json);
            // Add content-type header if not present
            let mut headers = self.headers.unwrap_or_default();
            if !headers.contains_key(CONTENT_TYPE) {
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
            }
            self.headers = Some(headers);
        }
        self
    }

    // For backward compatibility
    pub fn with_body(mut self, body: Value) -> Self {
        self.body = RequestBody::Json(body);
        self
    }

    pub fn form_body<K, V>(mut self, form_data: &[(K, V)]) -> Self 
    where 
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut map = HashMap::new();
        for (key, value) in form_data {
            map.insert(key.as_ref().to_string(), value.as_ref().to_string());
        }
        self.body = RequestBody::FormUrlEncoded(map);
        
        // Add content-type header
        let mut headers = self.headers.unwrap_or_default();
        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        self.headers = Some(headers);
        self
    }

    pub fn multipart_form(mut self, parts: Vec<(String, MultipartContent)>) -> Self {
        self.body = RequestBody::MultipartForm(parts);
        self
    }

    pub fn text_body<S: Into<String>>(mut self, text: S) -> Self {
        self.body = RequestBody::Text(text.into());
        let mut headers = self.headers.unwrap_or_default();
        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("text/plain"),
            );
        }
        self.headers = Some(headers);
        self
    }

    pub fn xml_body<S: Into<String>>(mut self, xml: S) -> Self {
        self.body = RequestBody::Xml(xml.into());
        let mut headers = self.headers.unwrap_or_default();
        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/xml"),
            );
        }
        self.headers = Some(headers);
        self
    }

    pub fn binary_body(mut self, data: Vec<u8>) -> Self {
        self.body = RequestBody::Binary(data);
        let mut headers = self.headers.unwrap_or_default();
        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            );
        }
        self.headers = Some(headers);
        self
    }

    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }
}

pub async fn make_request<T>(config: RequestConfig) -> Result<ActionResponse, String>
where
    T: DeserializeOwned + IntoActionResponse,
{
    let client = Client::new();
    
    // Build the request URL with query parameters
    let mut url = reqwest::Url::parse(&config.url)
        .map_err(|e| format!("Invalid URL: {}", e))?;
    
    if let Some(params) = config.params {
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.clear();
        for (key, value) in params {
            query_pairs.append_pair(&key, &value);
        }
    }

    // Build the request
    let mut request = client.request(config.method, url);

    // Add headers if provided
    if let Some(headers) = config.headers {
        request = request.headers(headers);
    }

    // Add authentication if provided
    if let Some(auth) = config.auth {
        request = match auth {
            Auth::Basic { username, password } => {
                request.basic_auth(username, password)
            },
            Auth::Bearer(token) => {
                request.bearer_auth(token)
            }
        };
    }

    // Add body based on type
    request = match config.body {
        RequestBody::Json(json) => request.json(&json),
        RequestBody::FormUrlEncoded(form_data) => request.form(&form_data),
        RequestBody::MultipartForm(parts) => {
            let mut form = Form::new();
            for (name, content) in parts {
                match content {
                    MultipartContent::File { path, filename, content_type } => {
                        let mut file = File::open(path)
                            .map_err(|e| format!("Failed to open file: {}", e))?;
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer)
                            .map_err(|e| format!("Failed to read file: {}", e))?;
                        let part = Part::bytes(buffer)
                            .file_name(filename.unwrap_or_else(|| "file".to_string()));
                        if let Some(ct) = content_type {
                            form = form.part(name, part.mime_str(&ct).unwrap());
                        } else {
                            form = form.part(name, part);
                        }
                    },
                    MultipartContent::Text { content, content_type } => {
                        let part = Part::text(content);
                        if let Some(ct) = content_type {
                            form = form.part(name, part.mime_str(&ct).unwrap());
                        } else {
                            form = form.part(name, part);
                        }
                    },
                    MultipartContent::Bytes { content, filename, content_type } => {
                        let mut part = Part::bytes(content);
                        if let Some(fname) = filename {
                            part = part.file_name(fname);
                        }
                        if let Some(ct) = content_type {
                            form = form.part(name, part.mime_str(&ct).unwrap());
                        } else {
                            form = form.part(name, part);
                        }
                    }
                }
            }
            request.multipart(form)
        },
        RequestBody::Text(text) => request.body(text),
        RequestBody::Xml(xml) => request.body(xml),
        RequestBody::Binary(data) => request.body(data),
        RequestBody::None => request,
    };

    // Send the request and handle the response
    let response = request.send().await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<T>().await {
                    Ok(parsed_response) => Ok(parsed_response.into_action_response()),
                    Err(err) => {
                        let err = format!("Failed to parse JSON response: {}", err);
                        Err(err)
                    }
                }
            } else {
                let err = format!(
                    "Request failed with status: {} and message: {}",
                    resp.status(),
                    resp.text().await.unwrap_or_else(|_| "Unknown error".to_string())
                );
                Err(err)
            }
        },
        Err(err) => match err.status() {
            Some(status) => {
                let err = format!("Error: {} with status code: {}", err, status);
                Err(err)
            },
            None => {
                let err = format!("Error: {}", err);
                Err(err)
            }
        }
    }
}