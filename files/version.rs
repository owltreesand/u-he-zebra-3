//! Static build info for U He Zebra 3.

pub const BUILD_VERSION: &str  = "v4.2.23";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "eb4835a89af7";
pub const BUILD_TIMESTAMP: u64 = 1782799145;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "U He Zebra 3", BUILD_VERSION, BUILD_CHANNEL)
}
