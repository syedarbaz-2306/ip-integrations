use reqwest::{
    Client, Method, 
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum RequestBody {
    Json(Value),
    FormUrlEncoded(HashMap<String, String>),
    Multipart(reqwest::multipart::Form),
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

    pub fn params<K, V, I>(mut self, params: I) -> Self 
    where 
        K: AsRef<str>,
        V: AsRef<str>,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = HashMap::new();
        for (key, value) in params.into_iter() {
            map.insert(key.as_ref().to_string(), value.as_ref().to_string());
        }
        self.params = Some(map);
        self
    }

    pub fn headers<K, V, I>(mut self, headers: I) -> Self 
    where 
        K: AsRef<str>,
        V: AsRef<str>,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers.into_iter() {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_bytes(key.as_ref().as_bytes()),
                HeaderValue::from_str(value.as_ref()),
            ) {
                header_map.insert(name, val);
            }
        }
        self.headers = Some(header_map);
        self
    }

    pub fn json_body<T: Serialize>(mut self, body: T) -> Self {
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

    pub fn form_body<K, V, I>(mut self, form_data: I) -> Self 
    where 
        K: AsRef<str>,
        V: AsRef<str>,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = HashMap::new();
        for (key, value) in form_data.into_iter() {
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

    pub fn multipart_body(mut self, form: reqwest::multipart::Form) -> Self {
        self.body = RequestBody::Multipart(form);
        self
    }

    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }
}

pub async fn make_request(config: RequestConfig) -> Result<Option<Value>, Value> {
    let client = Client::new();
    
    let mut url = reqwest::Url::parse(&config.url)
        .map_err(|e| Value::String(format!("Invalid URL: {}", e)))?;
    
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
        RequestBody::Multipart(form) => request.multipart(form),
        RequestBody::None => request,
    };

    let response = request.send().await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                if status == reqwest::StatusCode::NO_CONTENT {
                    Ok(None)
                } else {
                    match resp.json::<Value>().await {
                        Ok(value) => Ok(Some(value)),
                        Err(e) => Err(Value::String(format!("Failed to parse response JSON: {}", e))),
                    }
                }
            } else {
                let text = resp.text().await.unwrap_or_default();
                match serde_json::from_str(&text) {
                    Ok(value) => Err(value),
                    Err(_) => {
                        let error_value = serde_json::json!({
                            "status": status.as_u16(),
                            "message": text,
                        });
                        Err(error_value)
                    }
                }
            }
        },
        Err(err) => {
            let status_code = err.status().map(|s| s.as_u16()).unwrap_or(0);
            let message = err.to_string();
            let error_value = serde_json::json!({
                "status": status_code,
                "message": message,
            });
            Err(error_value)
        }
    }
}