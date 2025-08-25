//! # App structure
//!
//! This module provides the `ARGS` static structure.
//!
//! This module structures the argument-command
//! pattern for this binary.

use std::sync::LazyLock;

use clap::Parser;
use derive_getters::{Dissolve, Getters};

/// A reuseable `'static` variable for argument parsing memoization.
///
/// By using this, you only parse the binary arguments once!
pub static ARGS: LazyLock<Args> = LazyLock::new(|| Args::parse());

/// Automatic API command querying
#[derive(Parser, Clone, Debug, Dissolve, Getters)]
#[command(name = "traffic_cone", version, about, long_about = None)]
pub struct Args {
    /// Operation mode
    #[command(subcommand)]
    mode: Mode,

    /// Path to the api key
    #[arg(short = 'k', long = "key-path")]
    api_key_path: String,
}

/// The API method call
#[derive(Parser, Clone, Debug)]
pub enum Mode {
    /// All download commands
    #[command(subcommand)]
    Downloads(Download),
    /// All hosts commands
    #[command(subcommand)]
    Hosts(Hosts),
    /// All torrent commands
    #[command(subcommand)]
    Torrents(Torrents),
}

/// All download commands
#[derive(Parser, Clone, Debug)]
pub enum Download {
    /// Get raw JSON output
    Json,
    /// Delete a specific video.
    Delete {
        /// Video ID to be deleted
        id: String,
    },
}
impl From<Download> for Mode {
    fn from(value: Download) -> Self {
        Mode::Downloads(value)
    }
}

/// All hosts commands
#[derive(Parser, Clone, Debug)]
pub enum Hosts {
    /// Get raw JSON output.
    Json,
}
impl From<Hosts> for Mode {
    fn from(value: Hosts) -> Self {
        Mode::Hosts(value)
    }
}

/// All hosts commands
#[derive(Parser, Clone, Debug)]
pub enum Torrents {
    /// Get list of json metadata
    List,

    /// Add magnet to torrent
    AddMagnet {
        /// The link for the magnet
        link: String,
    },
}
impl From<Torrents> for Mode {
    fn from(value: Torrents) -> Self {
        Mode::Torrents(value)
    }
}
