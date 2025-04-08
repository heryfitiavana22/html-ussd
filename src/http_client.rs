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
            Err(err) => Err(format!("Failed to fetch remote page: {:?}", err)),
        }
    }
}
