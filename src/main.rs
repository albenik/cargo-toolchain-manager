//! # cargo-toolchain
//!
//! `cargo-toolchain` is a command-line utility that helps you install
//! multiple versions of the Rust compiler using `rustup`.
//!
//! It supports:
//! - Installing all stable versions
//! - Filtering with semantic version ranges
//! - Installing from `stable`, `beta`, `nightly`, or all channels
//! - Dry-run mode
//! - Skipping already installed toolchains
//! - Caching version index from the official JSON feed
//!
//! ## Example usage
//!
//! ```bash
//! cargo-toolchain --range ">=1.60.0, <1.70.0" --dry-run
//! ```
//!
//! ## Requirements
//!
//! - `rustup` must be installed
//!
//! ## License
//! MIT

mod releases;

use std::error::Error;
use std::process::Command;

use clap::Parser;
use semver::{Version, VersionReq};

use crate::releases::parse_releases_md;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Only display output without installing
    #[arg(long)]
    dry_run: bool,

    /// Semver version range (e.g., '>=1.60.0, <1.70.0')
    #[arg(long)]
    range: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let range_filter = args.range.as_ref().and_then(|r| VersionReq::parse(r).ok());

    let versions = parse_releases_md().await?;
    let filtered: Vec<Version> = versions
        .into_iter()
        .filter(|v| range_filter.as_ref().is_none_or(|req| req.matches(v)))
        .collect();

    for version in &filtered {
        let version_str = format!("{}.{}", version.major, version.minor);
        println!("Install {version_str}");

        if !args.dry_run {
            let mut child = Command::new("rustup")
                .arg("toolchain")
                .arg("install")
                .arg(version_str)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("Failed to start rustup");

            let status = child.wait().expect("Failed to wait on rustup");

            if !status.success() {
                eprintln!("❌ Unable to install {}", version);
            }
        }
    }

    println!("🎉 Done");
    Ok(())
}
