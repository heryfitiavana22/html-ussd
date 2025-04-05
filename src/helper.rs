pub fn is_server_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn muted_text(input: &str) -> String {
    format!("\x1b[90m{} > \x1b[0m", input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_server_url_with_http() {
        assert!(is_server_url("http://example.com"));
    }

    #[test]
    fn test_is_server_url_with_https() {
        assert!(is_server_url("https://example.com"));
    }

    #[test]
    fn test_is_server_url_with_no_protocol() {
        assert!(!is_server_url("example.com"));
    }

    #[test]
    fn test_is_server_url_with_ftp_protocol() {
        assert!(!is_server_url("ftp://example.com"));
    }

    #[test]
    fn test_is_server_url_with_empty_string() {
        assert!(!is_server_url(""));
    }

    #[test]
    fn test_is_server_url_with_whitespace() {
        assert!(!is_server_url("   "));
    }

    #[test]
    fn test_is_server_url_with_partial_http() {
        assert!(!is_server_url("http:/example.com"));
    }

    #[test]
    fn test_is_server_url_with_partial_https() {
        assert!(!is_server_url("https:/example.com"));
    }
}
