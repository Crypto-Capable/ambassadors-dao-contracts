use crate::*;

pub fn assert_valid_resource_url(url: &str) {
    let url = url.trim();
    assert!(!url.is_empty(), "{}", error::ERR_INVALID_RESOURCE_URL);
    assert!(
        url.starts_with("http://") || url.starts_with("https://"),
        "{}",
        error::ERR_INVALID_RESOURCE_URL
    );
}
