use reqwest::{
    Client, Method, 
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use super::{action_response::ActionResponse, into_action_response::IntoActionResponse};

#[derive(Debug)]
pub enum RequestBody {
    Json(Value),
    FormUrlEncoded(HashMap<String, String>),
    None,
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

    pub fn with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params = Some(params);
        self
    }

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

    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn json_body<T: serde::Serialize>(mut self, body: T) -> Self {
        if let Ok(json) = serde_json::to_value(body) {
            self.body = RequestBody::Json(json);
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
    
    let mut url = reqwest::Url::parse(&config.url)
        .map_err(|e| format!("Invalid URL: {}", e))?;
    
    if let Some(params) = config.params {
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.clear();
        for (key, value) in params {
            query_pairs.append_pair(&key, &value);
        }
    }

    let mut request = client.request(config.method, url);

    if let Some(headers) = config.headers {
        request = request.headers(headers);
    }

    if let Some(auth) = config.auth {
        request = match auth {
            Auth::Basic { username, password } => request.basic_auth(username, password),
            Auth::Bearer(token) => request.bearer_auth(token),
        };
    }

    request = match config.body {
        RequestBody::Json(json) => request.json(&json),
        RequestBody::FormUrlEncoded(form_data) => request.form(&form_data),
        RequestBody::None => request,
    };

    let response = request.send().await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<T>().await {
                    Ok(parsed_response) => Ok(parsed_response.into_action_response()),
                    Err(err) => Err(format!("Failed to parse JSON response: {}", err)),
                }
            } else {
                Err(format!(
                    "Request failed with status: {} and message: {}",
                    resp.status(),
                    resp.text().await.unwrap_or_else(|_| "Unknown error".to_string())
                ))
            }
        },
        Err(err) => match err.status() {
            Some(status) => Err(format!("Error: {} with status code: {}", err, status)),
            None => Err(format!("Error: {}", err)),
        }
    }
}