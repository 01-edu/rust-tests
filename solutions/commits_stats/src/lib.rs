use std::collections::HashMap;

use chrono::{DateTime, Datelike};

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    data.members()
        .map(|l| l["commit"]["author"]["date"].to_string())
        .fold(HashMap::new(), |mut acc, x| {
            acc.entry(format!(
                "{:?}",
                DateTime::parse_from_rfc3339(x.as_str()).unwrap().iso_week()
            ))
            .and_modify(|v| *v += 1)
            .or_insert(1);

            acc
        })
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    data.members()
        .map(|l| l["author"]["login"].to_string())
        .fold(HashMap::new(), |mut acc, x| {
            acc.entry(x).and_modify(|v| *v += 1).or_insert(1);

            acc
        })
}
