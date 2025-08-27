#![allow(unused_variables)]

use crate::app::*;
use crate::prelude::*;

pub(crate) fn handle_user(entry: User) -> ! {
    use crate::user::*;
    use User::*;

    let response_body = match entry {
        Json => get_user(),
    };

    println!("{response_body}");

    exit(0)
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
    use crate::traffic::*;
    use Traffic::*;

    let response_body = match entry {
        Json => get_traffic(),
        Details => get_details(),
    };

    println!("{response_body}");

    exit(0)
}

pub(crate) fn handle_streaming(entry: Streaming) -> ! {
    use crate::streaming::*;
    use Streaming::*;

    let response_body = match entry {
        Transcode { id } => transcode(id),
        MediaInfos { id } => media_infos(id),
    };

    println!("{response_body}");

    exit(0)
}

pub(crate) fn handle_downloads(entry: Download) -> ! {
    use crate::download::*;
    use Download::*;

    let response_body = match entry {
        Json => get_downloads(),
        Delete { id } => delete_download(id),
    };

    println!("{response_body}");

    exit(0)
}

pub(crate) fn handle_torrents(entry: Torrents) -> ! {
    use crate::torrents::*;
    use Torrents::*;

    let response_body = match entry {
        Json => get_torrents(),
        Info { id } => get_torrent_info(id),
        ActiveCount => get_active_count(),
        AvailableHosts => get_available_hosts(),
        AddTorrent { host } => add_torrent(host),
        AddMagnet { link } => add_magnet(link),
        SelectFiles { id, files } => select_files(id, files),
        Delete { id } => delete(id),
    };

    println!("{response_body}");

    exit(0)
}

pub(crate) fn handle_hosts(entry: Hosts) -> ! {
    use crate::hosts::*;
    use Hosts::*;

    let response_body = match entry {
        Json => get_hosts(),
        Status => get_status(),
        Regex => get_regex(),
        RegexFolder => get_regex_folder(),
        Domains => get_domains(),
    };

    println!("{response_body}");

    exit(0)
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
