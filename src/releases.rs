use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use itertools::Itertools;
use regex::RegexBuilder;
use semver::Version;

pub async fn parse_releases_md() -> Result<Vec<Version>, Box<dyn Error>> {
    let body = reqwest::get("https://raw.githubusercontent.com/rust-lang/rust/stable/RELEASES.md")
        .await?
        .error_for_status()?
        .text()
        .await?;

    let split_re = RegexBuilder::new("^Version\\s+")
        .multi_line(true)
        .build()
        .unwrap();

    let versions = split_re
        .split(&body)
        .filter_map(|s| {
            if let Some(ws_idx) = s.find(|c: char| c.is_whitespace()) {
                let version = &s[0..ws_idx];
                if let Ok(version) = Version::parse(version) {
                    if version > Version::from_str("1.0.0").unwrap() {
                        Some(version)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sorted();

    let mut latest_versions: HashMap<(u64, u64), Version> = HashMap::new();

    for version in versions {
        let key = (version.major, version.minor);

        latest_versions
            .entry(key)
            .and_modify(|existing| {
                if &version > existing {
                    *existing = version.clone();
                }
            })
            .or_insert(version);
    }

    let versions: Vec<Version> = latest_versions.values().sorted().cloned().collect();
    Ok(versions)
}
