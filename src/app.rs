#![allow(unused_variables)]
//! # App structure
//!
//! This module provides the `ARGS` static structure.
//!
//! This module structures the argument-command
//! pattern for this binary.

use std::sync::LazyLock;

use clap::Parser;
use derive_getters::{Dissolve, Getters};

use crate::prelude::*;

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
    Check,
    Link,
    Folder,
    ContainerFile,
    ContainerLink,
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
    AddTorrent,
    /// Add magnet to torrent
    AddMagnet {
        /// The link for the magnet
        link: String,
    },
    SelectFiles {
        id: String,
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
    Update,
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

pub(crate) fn handle_user(entry: User) -> ! {
    use User::*;

    match entry {
        Json => todo!(),
    }
}

pub(crate) fn handle_unrestrict(entry: Unrestrict) -> ! {
    use Unrestrict::*;

    match entry {
        Check => todo!(),
        Link => todo!(),
        Folder => todo!(),
        ContainerFile => todo!(),
        ContainerLink => todo!(),
    }
}

pub(crate) fn handle_traffic(entry: Traffic) -> ! {
    use Traffic::*;

    match entry {
        Json => todo!(),
        Details => todo!(),
    }
}

pub(crate) fn handle_streaming(entry: Streaming) -> ! {
    use Streaming::*;

    match entry {
        Transcode { id } => todo!(),
        MediaInfos { id } => todo!(),
    }
}

pub(crate) fn handle_downloads(entry: Download) -> ! {
    use Download::*;
    match entry {
        Json => {
            println!("{}", crate::download::get_downloads());
            exit(0)
        }
        Delete { id } => {
            println!(
                "{}",
                String::from(crate::download::delete_download(id.clone()))
            );
            exit(0)
        }
    }
}

pub(crate) fn handle_torrents(entry: Torrents) -> ! {
    use Torrents::*;

    match entry {
        Json => {
            println!("{}", crate::torrents::get_torrents());
            exit(0)
        }
        Info { id } => todo!(),
        ActiveCount => todo!(),
        AvailableHosts => todo!(),
        AddTorrent => todo!(),
        AddMagnet { link } => {
            println!("{}", crate::torrents::add_magnet(link));
            exit(0)
        }
        SelectFiles { id } => todo!(),
        Delete { id } => todo!(),
    }
}

pub(crate) fn handle_hosts(entry: Hosts) -> ! {
    use Hosts::*;

    match entry {
        Json => {
            println!("{}", crate::hosts::get_hosts());
            exit(0)
        }
        Status => todo!(),
        Regex => todo!(),
        RegexFolder => todo!(),
        Domains => todo!(),
    }
}

pub(crate) fn handle_settings(entry: Settings) -> ! {
    use Settings::*;

    match entry {
        Json => todo!(),
        Update => todo!(),
        ConvertPoints => todo!(),
        ChangePassword => todo!(),
        AvatarFile => todo!(),
        AvatarDelete => todo!(),
    }
}

pub fn handle_mode(entry: Mode) -> ! {
    use Mode::*;

    match entry {
        User(user) => handle_user(user),
        Unrestrict(unrestrict_command) => handle_unrestrict(unrestrict_command),
        Traffic(traffic_command) => handle_traffic(traffic_command),
        Streaming(streaming_command) => handle_streaming(streaming_command),
        Downloads(download_command) => handle_downloads(download_command),
        Torrents(torrent_command) => handle_torrents(torrent_command),
        Hosts(host_command) => handle_hosts(host_command),
        Settings(setting_command) => handle_settings(setting_command),
    }
}
