use std::process::Command;

use clap::error::Result;

pub fn is_server_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn muted_text(input: &str) -> String {
    format!("\x1b[90m{} > \x1b[0m", input)
}

pub fn load_file(file_path: &str) -> Result<String, String> {
    let content = std::fs::read_to_string(file_path);
    match content {
        Ok(content) => Ok(content),
        Err(err) => Err(format!("Error reading file {}: {}", file_path, err)),
    }
}

pub fn parse_key_value_safe(pairs: &[String]) -> Result<Vec<(String, String)>, String> {
    let mut result = Vec::new();

    for pair in pairs {
        if let Some((key, value)) = pair.split_once('=') {
            result.push((key.to_string(), value.to_string()));
        } else {
            return Err(format!(
                "Invalid key-value pair: '{}'. Expected format 'key=value'.",
                pair
            ));
        }
    }

    Ok(result)
}

pub fn uninstall_self() {
    if cfg!(target_os = "windows") {
        use std::env;
        let local_app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
        let path = format!(r"{}\Programs\html-ussd\bin\html-ussd.exe", local_app_data);

        match std::fs::remove_file(&path) {
            Ok(_) => {
                println!("Successfully uninstalled html-ussd from {}", path);
            }
            Err(err) => {
                eprintln!("Failed to uninstall: {}", err);
            }
        }
        return;
    }

    let path = "/usr/local/bin/html-ussd";
    let status = Command::new("sudo").arg("rm").arg(path).status();

    match status {
        Ok(s) if s.success() => {
            println!("Successfully uninstalled html-ussd from {}", path);
        }
        Ok(_) | Err(_) => {
            eprintln!("Failed to uninstall.");
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

    #[test]
    fn test_parse_key_value_safe_with_valid_pairs() {
        let input = vec![
            "key1=value1".to_string(),
            "key2=value2".to_string(),
            "key3=value3".to_string(),
        ];
        let expected = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ];
        assert_eq!(parse_key_value_safe(&input), Ok(expected));
    }

    #[test]
    fn test_parse_key_value_safe_with_invalid_pair() {
        let input = vec![
            "key1=value1".to_string(),
            "invalidpair".to_string(),
            "key2=value2".to_string(),
        ];
        assert!(parse_key_value_safe(&input).is_err());
    }

    #[test]
    fn test_parse_key_value_safe_with_empty_input() {
        let input: Vec<String> = vec![];
        let expected: Vec<(String, String)> = vec![];
        assert_eq!(parse_key_value_safe(&input), Ok(expected));
    }

    #[test]
    fn test_parse_key_value_safe_with_empty_key() {
        let input = vec!["=value".to_string()];
        let expected = vec![("".to_string(), "value".to_string())];
        assert_eq!(parse_key_value_safe(&input), Ok(expected));
    }

    #[test]
    fn test_parse_key_value_safe_with_empty_value() {
        let input = vec!["key=".to_string()];
        let expected = vec![("key".to_string(), "".to_string())];
        assert_eq!(parse_key_value_safe(&input), Ok(expected));
    }

    #[test]
    fn test_parse_key_value_safe_with_empty_key_and_value() {
        let input = vec!["=".to_string()];
        let expected = vec![("".to_string(), "".to_string())];
        assert_eq!(parse_key_value_safe(&input), Ok(expected));
    }

    #[test]
    fn test_parse_key_value_safe_with_multiple_equals() {
        let input = vec!["key=value=extra".to_string()];
        let expected = vec![("key".to_string(), "value=extra".to_string())];
        assert_eq!(parse_key_value_safe(&input), Ok(expected));
    }
}
