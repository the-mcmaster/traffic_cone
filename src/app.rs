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

    /// Only print successful information.
    ///
    /// Disable's in-app stderr.
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

/// The API method call
#[derive(Parser, Clone, Debug)]
pub enum Mode {
    /// All user commands
    #[command(subcommand)]
    User(User),
    /// All unrestrict commands
    #[command(subcommand)]
    Unrestrict(Unrestrict),
    /// All traffic commands
    #[command(subcommand)]
    Traffic(Traffic),
    /// All streaming commands
    #[command(subcommand)]
    Streaming(Streaming),
    /// All download commands
    #[command(subcommand)]
    Downloads(Download),
    /// All torrent commands
    #[command(subcommand)]
    Torrents(Torrents),
    /// All hosts commands
    #[command(subcommand)]
    Hosts(Hosts),
    /// All settings commands
    #[command(subcommand)]
    Settings(Settings),
}

/// All user commands
#[derive(Parser, Clone, Debug)]
pub enum User {
    /// Get raw JSON output
    Json,
}
impl From<User> for Mode {
    fn from(value: User) -> Self {
        Mode::User(value)
    }
}

/// All unrestrict commands
#[derive(Parser, Clone, Debug)]
pub enum Unrestrict {
    Check {link: String},
    Link {link: String},
    Folder {link: String},
    ContainerFile,
    ContainerLink {link: String},
}
impl From<Unrestrict> for Mode {
    fn from(value: Unrestrict) -> Self {
        Mode::Unrestrict(value)
    }
}

/// All traffic commands
#[derive(Parser, Clone, Debug)]
pub enum Traffic {
    Json,
    Details,
}
impl From<Traffic> for Mode {
    fn from(value: Traffic) -> Self {
        Mode::Traffic(value)
    }
}

/// All streaming commands
#[derive(Parser, Clone, Debug)]
pub enum Streaming {
    Transcode { id: String },
    MediaInfos { id: String },
}
impl From<Streaming> for Mode {
    fn from(value: Streaming) -> Self {
        Mode::Streaming(value)
    }
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

/// All torrents commands
#[derive(Parser, Clone, Debug)]
pub enum Torrents {
    /// Get list of json metadata
    Json,

    Info {
        id: String,
    },

    ActiveCount,
    AvailableHosts,
    AddTorrent {
        host: String,
    },
    /// Add magnet to torrent
    AddMagnet {
        /// The link for the magnet
        link: String,
    },
    SelectFiles {
        id: String,
        files: String,
    },
    Delete {
        id: String,
    },
}
impl From<Torrents> for Mode {
    fn from(value: Torrents) -> Self {
        Mode::Torrents(value)
    }
}

/// All hosts commands
#[derive(Parser, Clone, Debug)]
pub enum Hosts {
    /// Get raw JSON output.
    Json,
    Status,
    Regex,
    RegexFolder,
    Domains,
}
impl From<Hosts> for Mode {
    fn from(value: Hosts) -> Self {
        Mode::Hosts(value)
    }
}

/// All hosts commands
#[derive(Parser, Clone, Debug)]
pub enum Settings {
    /// Get raw JSON output.
    Json,
    Update {setting_name: String, setting_value: String},
    ConvertPoints,
    ChangePassword,
    AvatarFile,
    AvatarDelete,
}
impl From<Settings> for Mode {
    fn from(value: Settings) -> Self {
        Mode::Settings(value)
    }
}
