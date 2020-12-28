use clap::{App, Arg, ArgGroup, SubCommand};

fn device_arg() -> Arg<'static, 'static> {
  Arg::with_name("device")
    .short("d")
    .long("device")
    .takes_value(true)
    .value_name("DEVICE")
    .help("Specifies the spotify device to use")
}

fn format_arg() -> Arg<'static, 'static> {
  Arg::with_name("format")
    .short("f")
    .long("format")
    .takes_value(true)
    .value_name("FORMAT")
    .help("Specifies the output format ('--help' for more information)")
    .long_help(
      "There are multiple format specifiers you can use: \
%a: artist, %b: album, %p: playlist, %t: track, %h: show, \
%f: flags (shuffle, repeat, like), %s: playback status, \
%v: volume, %d: current device. Example: \
spt pb -s -f 'playing on %d at %v%'",
    )
}

pub fn playback_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("playback")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Interacts with the playback of a device")
    .visible_alias("pb")
    .arg(device_arg())
    .arg(
      format_arg()
        .default_value("%f %s %t - %a")
        .default_value_ifs(&[
          ("volume", None, "%v% %f %s %t - %a"),
          ("transfer", None, "%f %s %t - %a on %d"),
        ]),
    )
    .arg(
      Arg::with_name("toggle")
        .short("t")
        .long("toggle")
        .help("Pauses/resumes the playback of a device"),
    )
    .arg(
      Arg::with_name("status")
        .short("s")
        .long("status")
        .help("Prints out the current status of a device"),
    )
    .arg(
      Arg::with_name("share-track")
        .long("share-track")
        .help("Returns the url to the current track"),
    )
    .arg(
      Arg::with_name("share-album")
        .long("share-album")
        .help("Returns the url to the album of the current track"),
    )
    .arg(
      Arg::with_name("transfer")
        .long("transfer")
        .takes_value(true)
        .value_name("DEVICE")
        .help("Transfers the playback to new DEVICE"),
    )
    .arg(
      Arg::with_name("like")
        .long("like")
        .help("Likes the current song"),
    )
    .arg(
      Arg::with_name("shuffle")
        .long("shuffle")
        .help("Toggles shuffle mode"),
    )
    .arg(
      Arg::with_name("repeat")
        .long("repeat")
        .help("Switches between repeat modes"),
    )
    .arg(
      Arg::with_name("next")
        .short("n")
        .long("next")
        .multiple(true)
        .help("Jumps to the next song (can be chained: -n -n -n ...)"),
    )
    .arg(
      Arg::with_name("previous")
        .short("p")
        .long("previous")
        .multiple(true)
        .help("Jumps to the previous song (can be chained: -p -p -p ...)"),
    )
    .arg(
      Arg::with_name("volume")
        .short("v")
        .long("volume")
        .takes_value(true)
        .value_name("VOLUME")
        .help("Sets the volume of a device to VOLUME (1 - 100)"),
    )
    .group(
      ArgGroup::with_name("jumps")
        .args(&["next", "previous"])
        .multiple(false)
        .conflicts_with_all(&["single", "flags", "actions"]),
    )
    .group(
      ArgGroup::with_name("flags")
        .args(&["like", "shuffle", "repeat"])
        .multiple(true)
        .conflicts_with_all(&["single", "jumps"]),
    )
    .group(
      ArgGroup::with_name("actions")
        .args(&["toggle", "status", "transfer", "volume"])
        .multiple(true)
        .conflicts_with_all(&["single", "jumps"]),
    )
    .group(
      ArgGroup::with_name("single")
        .args(&["share-track", "share-album"])
        .multiple(false)
        .conflicts_with_all(&["actions", "flags", "jumps"]),
    )
}

pub fn play_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("play")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Plays a uri or another spotify item by name")
    .visible_alias("p")
    .arg(device_arg())
    .arg(format_arg().default_value("%f %s %t - %a"))
    .arg(
      Arg::with_name("uri")
        .short("u")
        .long("uri")
        .takes_value(true)
        .value_name("URI")
        .help("Plays the URI"),
    )
    .arg(
      Arg::with_name("name")
        .short("n")
        .long("name")
        .takes_value(true)
        .value_name("NAME")
        .help("Plays the first match with NAME from the specified category"),
    )
    .arg(
      Arg::with_name("queue")
        .short("q")
        .long("queue")
        // Only works with tracks
        .conflicts_with_all(&["album", "artist", "playlist", "show"])
        .help("Adds track to queue instead of playing it directly"),
    )
    .arg(
      Arg::with_name("random")
        .short("r")
        .long("random")
        // Only works with playlists
        .conflicts_with_all(&["track", "album", "artist", "show"])
        .help("Plays a random track (only works with playlists)"),
    )
    .arg(
      Arg::with_name("album")
        .short("b")
        .long("album")
        .help("Looks for an album"),
    )
    .arg(
      Arg::with_name("artist")
        .short("a")
        .long("artist")
        .help("Looks for an artist"),
    )
    .arg(
      Arg::with_name("track")
        .short("t")
        .long("track")
        .help("Looks for a track"),
    )
    .arg(
      Arg::with_name("show")
        .short("w")
        .long("show")
        .help("Looks for a show"),
    )
    .arg(
      Arg::with_name("playlist")
        .short("p")
        .long("playlist")
        .help("Looks for a playlist"),
    )
    .group(
      ArgGroup::with_name("contexts")
        .args(&["track", "artist", "playlist", "album", "show"])
        .required(true)
        .multiple(false),
    )
    .group(
      ArgGroup::with_name("actions")
        .args(&["uri", "name"])
        .multiple(false)
        .required(true),
    )
}

pub fn list_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("list")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Lists devices, liked songs")
    .visible_alias("l")
    .arg(format_arg().default_value_ifs(&[
      ("devices", None, "%v% %d"),
      ("liked", None, "%t - %a (%u)"),
      ("playlists", None, "%p (%u)"),
    ]))
    .arg(
      Arg::with_name("devices")
        .short("d")
        .long("devices")
        .help("Lists devices"),
    )
    .arg(
      Arg::with_name("playlists")
        .short("p")
        .long("playlists")
        .help("Lists playlists"),
    )
    .arg(
      Arg::with_name("liked")
        .long("liked")
        .help("Lists liked songs"),
    )
    .arg(
      Arg::with_name("limit")
        .long("limit")
        .takes_value(true)
        .help("Specifies the maximum number of results (1 - 50)"),
    )
    .group(
      ArgGroup::with_name("listable")
        .args(&["devices", "playlists", "liked"])
        .required(true)
        .multiple(false),
    )
}

pub fn search_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("search")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Searches for tracks, albums and more")
    .visible_alias("s")
    .arg(format_arg().default_value_ifs(&[
      ("tracks", None, "%t - %a (%u)"),
      ("playlists", None, "%p (%u)"),
      ("artists", None, "%a (%u)"),
      ("albums", None, "%b - %a (%u)"),
      ("shows", None, "%h - %a (%u)"),
    ]))
    .arg(
      Arg::with_name("search")
        .required(true)
        .takes_value(true)
        .value_name("SEARCH")
        .help("Specifies the search query"),
    )
    .arg(
      Arg::with_name("albums")
        .short("b")
        .long("albums")
        .help("Looks for albums"),
    )
    .arg(
      Arg::with_name("artists")
        .short("a")
        .long("artists")
        .help("Looks for artists"),
    )
    .arg(
      Arg::with_name("playlists")
        .short("p")
        .long("playlists")
        .help("Looks for playlists"),
    )
    .arg(
      Arg::with_name("tracks")
        .short("t")
        .long("tracks")
        .help("Looks for tracks"),
    )
    .arg(
      Arg::with_name("shows")
        .short("w")
        .long("shows")
        .help("Looks for shows"),
    )
    .arg(
      Arg::with_name("limit")
        .long("limit")
        .takes_value(true)
        .help("Specifies the maximum number of results (1 - 50)"),
    )
    .group(
      ArgGroup::with_name("searchable")
        .args(&["playlists", "tracks", "albums", "artists", "shows"])
        .required(true)
        .multiple(false),
    )
}
