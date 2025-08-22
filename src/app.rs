//! # App structure
//!
//! This module provides the `ARGS` static structure.
//!
//! This module structures the argument-command
//! pattern for this binary.

use std::{num::NonZero, sync::LazyLock};

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
    Hosts(Hosts)
}

/// All download commands
#[derive(Parser)]
#[derive(Clone, Debug)]
pub enum Download {
    /// Get raw JSON output
    Json,
    /// List all download metadata.
    List,
    /// Delete a specific video.
    Delete {
        /// Video ID to be deleted
        id: String,
    },
    /// Download from a link.
    Download {
        /// Video ID to be deleted
        link: String,
    },
    /// Get list of link(s).
    ///
    /// Lack of an argument will list them all by default.
    Links {
        /// Get nth link.
        ///
        /// Will list the download links otherwise
        n: Option<NonZero<usize>>,
    },
}
impl From<Download> for Mode {
    fn from(value: Download) -> Self {
        Mode::Downloads(value)
    }
}

/// All hosts commands
#[derive(Parser)]
#[derive(Clone, Debug)]
pub enum Hosts {
    /// Get raw JSON output.
    Json,
    /// List host names.
    List,
}
impl From<Hosts> for Mode {
    fn from(value: Hosts) -> Self {
        Mode::Hosts(value)
    }
}