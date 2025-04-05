pub fn is_server_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}
