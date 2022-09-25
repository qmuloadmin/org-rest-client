use reqwest;
use serde_json::{self, to_string_pretty, Value};

pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

#[derive(Debug)]
pub struct Error {
    pub status: u16,
    pub message: String,
}

pub type RestResult = Result<Value, Error>;

pub struct RestClient {
    client: reqwest::Client,
    verbose: bool,
}

impl RestClient {
    pub fn new(verbose: bool) -> Self {
        RestClient {
            client: reqwest::Client::new(),
            verbose,
        }
    }

    pub fn new_authed(jwt: String, verbose: bool) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "AUTHORIZATION",
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", &jwt)).unwrap(),
        );
        RestClient {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
            verbose,
        }
    }

    pub async fn get(&self, url: String) -> RestResult {
        self.call(Method::GET, url, None).await
    }

    pub async fn post(&self, url: String, body: Value) -> RestResult {
        self.call(Method::POST, url, Some(body)).await
    }

    pub async fn patch(&self, url: String, body: Value) -> RestResult {
        self.call(Method::PATCH, url, Some(body)).await
    }

    pub async fn delete(&self, url: String) -> RestResult {
        self.call(Method::DELETE, url, None).await
    }

    pub async fn call(&self, method: Method, url: String, body: Option<Value>) -> RestResult {
        let req = match method {
            Method::GET => {
                print!("GET ");
                self.client.get(&url)
            }
            Method::POST => {
                print!("POST ");
                self.client.post(&url)
            }
            Method::PATCH => {
                print!("PATCH ");
                self.client.patch(&url)
            }
            Method::DELETE => {
                print!("DELETE ");
                self.client.delete(&url)
            }
            Method::PUT => {
                print!("PUT ");
                self.client.put(&url)
            }
        };
        if self.verbose {
            println!("{}", url);
        }
        match if let Some(json) = body {
            if self.verbose {
                println!("{}", to_string_pretty(&json).unwrap());
            }
            req.json(&json).send().await
        } else {
            req.send().await
        } {
            Ok(response) => {
                if let Ok(json_resp) = response.json().await {
                    if self.verbose {
                        println!(
                            "Response:\n{}\n\n----\n",
                            to_string_pretty(&json_resp).unwrap()
                        );
                    }
                    Ok(json_resp)
                } else {
                    Ok(Value::Null)
                }
            }
            Err(err) => Err(Error {
                status: err.status().unwrap().as_u16(),
                message: err.to_string(),
            }),
        }
    }
}
