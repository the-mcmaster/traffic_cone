#![feature(never_type)]

use lib::app::{self, ARGS};
use lib::{download, hosts};
use std::process::exit;

fn main() -> ! {
    let handle_downloads = |entry: app::Download| -> ! {
        use lib::app::Download::*;
        match entry {
            Json => {
                println!("{}", download::get_downloads_json());
                exit(0)
            }
            List => {
                let download_list = download::get_downloads();

                for entry in download_list {
                    println!("`{}`:", entry.filename());
                    println!("\tid:   {}", entry.id());
                    println!("\tlink: {}", entry.link());
                    eprintln!();
                }

                exit(0)
            }
            Delete { id } => {
                println!("{}", String::from(download::delete_download(id.clone())));
                exit(0)
            }
            Download { link } => {
                println!("{:?}", download::download(link.clone()));
                exit(0)
            }
            Links { n: None } => {
                let downloads = download::get_downloads();
                for link in downloads.iter().map(|entry| entry.link()) {
                    println!("{link}")
                }
                exit(0)
            }
            Links { n: Some(index) } => {
                let downloads = download::get_downloads();
                let download = downloads.iter().skip(index.get() - 1).next();
                if let Some(entry) = download {
                    println!("{}", entry.link())
                }
                exit(0)
            }
        }
    };

    let handle_hosts = |entry: app::Hosts| -> ! {
        use app::Hosts::*;
        
        match entry {
            Json => {
                println!("{}", hosts::get_hosts_json());
                exit(0)
            },
            List => {
                for (host_name, _data) in hosts::get_hosts() {
                    println!("{}", host_name);
                }
                exit(0)
            }
        }
    };

    let handle_mode = |entry: app::Mode| -> ! {
        use app::Mode::*;

        match entry {
            Downloads(download_command) => handle_downloads(download_command.clone()),
            Hosts(host_command) => handle_hosts(host_command.clone())
        }
    };

    handle_mode(ARGS.mode().clone())
}
