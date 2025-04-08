use clap::error::Result;
use reqwest::{
    Error,
    blocking::{Client, Response},
};

pub fn is_server_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn muted_text(input: &str) -> String {
    format!("\x1b[90m{} > \x1b[0m", input)
}

pub fn fetch_page(url: &str, query: Vec<(String, String)>) -> Result<String, String> {
    let client = Client::new();
    let response = client.get(url).query(&query).send();
    handle_result_response(response)
}

pub fn load_file(file_path: &str) -> Result<String, String> {
    let content = std::fs::read_to_string(file_path);
    match content {
        Ok(content) => Ok(content),
        Err(err) => Err(format!("Error reading file {}: {}", file_path, err)),
    }
}

pub fn handle_result_response(response: Result<Response, Error>) -> Result<String, String> {
    match response {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(html) = response.text() {
                    Ok(html)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_server_url_with_http() {
        assert_eq!(is_server_url("http://example.com"), true);
    }

    #[test]
    fn test_is_server_url_with_https() {
        assert_eq!(is_server_url("https://example.com"), true);
    }

    #[test]
    fn test_is_server_url_with_no_protocol() {
        assert_eq!(is_server_url("example.com"), false);
    }

    #[test]
    fn test_is_server_url_with_ftp_protocol() {
        assert_eq!(is_server_url("ftp://example.com"), false);
    }

    #[test]
    fn test_is_server_url_with_empty_string() {
        assert_eq!(is_server_url(""), false);
    }

    #[test]
    fn test_is_server_url_with_whitespace() {
        assert_eq!(is_server_url("   "), false);
    }

    #[test]
    fn test_is_server_url_with_partial_http() {
        assert_eq!(is_server_url("http:/example.com"), false);
    }

    #[test]
    fn test_is_server_url_with_partial_https() {
        assert_eq!(is_server_url("https:/example.com"), false);
    }
}
