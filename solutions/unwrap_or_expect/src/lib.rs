pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_owned(),
        Security::Message => server.expect("ERROR: program stops").to_owned(),
        Security::Warning => server.unwrap_or("WARNING: check the server").to_owned(),
        Security::NotFound => server
            .map(String::from)
            .unwrap_or_else(|url| format!("Not found: {}", url)),
        Security::UnexpectedUrl => server.unwrap_err().to_owned(),
    }
}
