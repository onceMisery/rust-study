use crate::errors::AppError;

pub fn find_config(key: &str) -> Option<&'static str> {
    match key {
        "host" => Some("localhost"),
        "env" => Some("dev"),
        _ => None,
    }
}

pub fn parse_port(raw: &str) -> Result<u16, AppError> {
    raw.parse::<u16>().map_err(|_| AppError::InvalidPort)
}
