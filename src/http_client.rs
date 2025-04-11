use reqwest::{
    Error,
    blocking::{Client, RequestBuilder, Response},
    header::HeaderMap,
};

pub struct HttpClient {
    pub default_query: Vec<(String, String)>,
    pub default_header: HeaderMap,
}

impl HttpClient {
    pub fn new(default_query: Vec<(String, String)>, default_header: HeaderMap) -> Self {
        Self {
            default_query,
            default_header,
        }
    }

    pub fn get(&self, url: &str, query: Vec<(String, String)>) -> Result<String, String> {
        let client = Client::new();
        self.send_request(client.get(url).query(&query))
    }

    pub fn post(&self, url: &str, data: Vec<(String, String)>) -> Result<String, String> {
        let client = Client::new();
        self.send_request(client.post(url).form(&data))
    }

    pub fn send_request(&self, request: RequestBuilder) -> Result<String, String> {
        self.handle_result_response(
            request
                .headers(self.default_header.clone())
                .query(&self.default_query)
                .send(),
        )
    }

    pub fn handle_result_response(
        &self,
        response: Result<Response, Error>,
    ) -> Result<String, String> {
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(text) = response.text() {
                        Ok(text)
                    } else {
                        Err("Failed to read response body".to_string())
                    }
                } else {
                    Err(format!(
                        "HTTP request failed with status: {}",
                        response.status()
                    ))
                }
            }
            Err(err) => Err(format!(
                "Failed to send HTTP request: {}",
                self.format_reqwest_error(&err)
            )),
        }
    }

    fn format_reqwest_error(&self, err: &Error) -> String {
        if err.is_timeout() {
            "Request timed out.".to_string()
        } else if err.is_connect() {
            format!(
                "Failed to connect to the server at URL: {}",
                err.url()
                    .map(|url| url.as_str().to_string())
                    .unwrap_or_else(|| "unknown URL".to_string())
            )
        } else if err.is_request() {
            format!("Problem with building the request: {}", err)
        } else if err.is_status() {
            format!("Server returned an error status: {}", err)
        } else if err.is_decode() {
            format!("Failed to decode the response body: {}", err)
        } else {
            format!("Unexpected error: {}", err)
        }
    }
}
