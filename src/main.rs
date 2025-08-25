#![feature(never_type)]

use lib::app::{self, ARGS};
use lib::{download, hosts, torrents};
use std::process::exit;

fn main() -> ! {
    let handle_downloads = |entry: app::Download| -> ! {
        use lib::app::Download::*;
        match entry {
            Json => {
                println!(".{}", download::get_downloads());
                exit(0)
            }
            Delete { id } => {
                println!(".{}", String::from(download::delete_download(id.clone())));
                exit(0)
            }
        }
    };

    let handle_hosts = |entry: app::Hosts| -> ! {
        use app::Hosts::*;

        match entry {
            Json => {
                println!(".{}", hosts::get_hosts());
                exit(0)
            }
        }
    };

    let handle_torrents = |entry: app::Torrents| -> ! {
        use app::Torrents::*;

        match entry {
            List => {
                println!(".{}", torrents::get_torrents());
                exit(0)
            }
            AddMagnet { link } => {
                println!(".{}", torrents::add_magnet(link));
                exit(0)
            }
        }
    };

    let handle_mode = |entry: app::Mode| -> ! {
        use app::Mode::*;

        eprintln!("e: {}", "handling mode!");
        println!("p: {}", "handling mode!");
        match entry {
            Downloads(download_command) => handle_downloads(download_command.clone()),
            Hosts(host_command) => handle_hosts(host_command.clone()),
            Torrents(torrent_command) => handle_torrents(torrent_command.clone()),
        }
    };

    handle_mode(ARGS.mode().clone())
}
