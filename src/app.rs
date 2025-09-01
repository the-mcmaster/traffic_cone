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

    /// Turn off color output.
    /// 
    /// Stdout is already color-free.
    /// 
    /// This disables strerr color output.
    #[arg(short, long, default_value_t = true)]
    no_color: bool
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
    /// Returns information on the current user.
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
    /// Check if a file is downloadable on the concerned hoster.
    Check { link: String },
    /// Unrestrict a hoster link and get a new unrestricted link
    Link { link: String },
    /// Unrestrict a hoster folder link and get individual links.
    /// 
    /// This returns an empty array if no links found.
    Folder { link: String },
    /// Decrypt a container file (RSDF, CCF, CCF3, DLC)
    ContainerFile,
    /// Decrypt a container file from a link.
    ContainerLink { link: String },
}
impl From<Unrestrict> for Mode {
    fn from(value: Unrestrict) -> Self {
        Mode::Unrestrict(value)
    }
}

/// All traffic commands
#[derive(Parser, Clone, Debug)]
pub enum Traffic {
    /// Get traffic informations for limited hosters (limits, current usage, extra packages)
    Json,
    /// Get traffic details on each hoster used during a defined period
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
    /// Get transcoding links for given file, {id} from `downloads` or `unrestrict link`
    Transcode { id: String },
    /// Get detailled media informations for given file, {id} from `downloads` or `unrestrict-link`
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
    /// Get user downloads list
    Json,
    /// Delete a link from downloads list, returns 204 HTTP code
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
    /// Get user torrents list
    Json,
    /// Get all informations on the asked torrent
    Info {
        id: String,
    },
    /// Get currently active torrents number and the current maximum limit
    ActiveCount,
    /// Get available hosts to upload the torrent to
    AvailableHosts,
    /// Add a torrent file to download, return a 201 HTTP code
    AddTorrent {
        host: String,
    },
    /// Add a magnet link to download, return a 201 HTTP code
    AddMagnet {
        /// The link for the magnet
        link: String,
    },
    /// Select files of a torrent to start it, returns 204 HTTP code
    SelectFiles {
        id: String,
        files: String,
    },
    /// Delete a torrent from torrents list, returns 204 HTTP code
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
    /// Get supported hosts
    Json,
    /// Get all supported links Regex, useful to find supported links inside a document
    Status,
    /// Get all supported folder Regex, useful to find supported links inside a document
    Regex,
    /// Get all supported folder Regex, useful to find supported links inside a document
    RegexFolder,
    /// Get all hoster domains supported on the service
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
    /// Get current user settings with possible values to update
    Json,
    /// Update a user setting, returns 204 HTTP code
    Update {
        setting_name: String,
        setting_value: String,
    },
    /// Convert fidelity points, returns 204 HTTP code
    ConvertPoints,
    /// Send the verification email to change the password, returns 204 HTTP code
    ChangePassword,
    /// Upload a new user avatar image, returns 204 HTTP code
    AvatarFile,
    /// Reset user avatar image to default, returns 204 HTTP code
    AvatarDelete,
}
impl From<Settings> for Mode {
    fn from(value: Settings) -> Self {
        Mode::Settings(value)
    }
}
