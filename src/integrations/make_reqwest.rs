use reqwest::{Client, Method, header::HeaderMap};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use super::{action_response::ActionResponse, into_action_response::IntoActionResponse};

// Trait for types that can be converted to ActionResponse

#[derive(Debug)]
pub struct RequestConfig {
    pub url: String,
    pub method: Method,
    pub params: Option<HashMap<String, String>>,
    pub body: Option<Value>,
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
            body: None,
            headers: None,
            auth: None,
        }
    }

    pub fn with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params = Some(params);
        self
    }

    pub fn with_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
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

    // Add body if provided
    if let Some(body) = config.body {
        request = request.json(&body);
    }

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