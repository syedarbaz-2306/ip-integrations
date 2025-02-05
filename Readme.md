<!--? Here are multiple examples demonstrating how to use make_request with different configurations: -->

1. GET Request with Query Parameters

async fn main() {
    let config = RequestConfig::new("https://api.example.com/data", Method::GET)
        .params([("key1", "value1"), ("key2", "value2")]);

    match make_request(config).await {
        Ok(Some(response)) => println!("Response: {:?}", response),
        Ok(None) => println!("No content received"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}

2. POST Request with JSON Body

async fn main() {
    let json_body = json!({
        "name": "John Doe",
        "email": "john@example.com"
    });

    let config = RequestConfig::new("https://api.example.com/users", Method::POST)
        .json_body(json_body);

    match make_request(config).await {
        Ok(Some(response)) => println!("Response: {:?}", response),
        Ok(None) => println!("No content received"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}

3. POST Request with Form URL Encoded Body

async fn main() {
    let config = RequestConfig::new("https://api.example.com/login", Method::POST)
        .form_body([("username", "john_doe"), ("password", "securepass")]);

    match make_request(config).await {
        Ok(Some(response)) => println!("Response: {:?}", response),
        Ok(None) => println!("No content received"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}

4. GET Request with Custom Headers

async fn main() {
    let config = RequestConfig::new("https://api.example.com/secure-data", Method::GET)
        .headers([
            ("X-Custom-Header", "CustomValue"),
            ("User-Agent", "MyApp/1.0"),
        ]);

    match make_request(config).await {
        Ok(Some(response)) => println!("Response: {:?}", response),
        Ok(None) => println!("No content received"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}

5. GET Request with Basic Authentication

async fn main() {
    let config = RequestConfig::new("https://api.example.com/authenticated", Method::GET)
        .with_auth(Auth::Basic {
            username: "user123".into(),
            password: Some("pass123".into()),
        });

    match make_request(config).await {
        Ok(Some(response)) => println!("Response: {:?}", response),
        Ok(None) => println!("No content received"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}

6. GET Request with Bearer Token Authentication

async fn main() {
    let config = RequestConfig::new("https://api.example.com/protected", Method::GET)
        .with_auth(Auth::Bearer("your-access-token".into()));

    match make_request(config).await {
        Ok(Some(response)) => println!("Response: {:?}", response),
        Ok(None) => println!("No content received"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}


