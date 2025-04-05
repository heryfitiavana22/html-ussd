use clap::error::Result;
use reqwest::{
    Error,
    blocking::{Response, get},
};

pub fn is_server_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn muted_text(input: &str) -> String {
    format!("\x1b[90m{} > \x1b[0m", input)
}

pub fn fetch_html(url: &str) -> Result<String, String> {
    let response = get(url);
    handle_result_response(response)
}

pub fn handle_result_response(response: Result<Response, Error>) -> Result<String, String> {
    match response {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(html) = response.text() {
                    return Ok(html);
                } else {
                    return Err(format!("Failed to read response body"));
                }
            } else {
                return Err(format!(
                    "HTTP request failed with status: {}",
                    response.status()
                ));
            }
        }
        Err(err) => {
            return Err(format!("Failed to fetch remote page: {:?}", err));
        }
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
