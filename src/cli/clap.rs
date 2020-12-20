use clap::{App, Arg, ArgGroup, SubCommand};

fn device_arg() -> Arg<'static, 'static> {
  Arg::with_name("device")
    .short("d")
    .long("device")
    .takes_value(true)
    .value_name("DEVICE")
    .help("Specify device to use")
}

fn format_arg() -> Arg<'static, 'static> {
  Arg::with_name("format")
    .short("f")
    .long("format")
    .takes_value(true)
    .value_name("FORMAT")
    .help("Specify output format")
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
    .about("Interact with playback of a device")
    .visible_alias("pb")
    .arg(device_arg())
    .arg(
      format_arg()
        .default_value("%f %s %t - %a")
        .default_value_ifs(&[
          ("volume", None, "%v% %s %t - %a"),
          ("transfer", None, "%f %s %t - %a on %d"),
        ]),
    )
    .arg(
      Arg::with_name("toggle")
        .short("t")
        .long("toggle")
        .help("Pause/resume playback of a device"),
    )
    .arg(
      Arg::with_name("status")
        .short("s")
        .long("status")
        .help("Print out status of a device"),
    )
    .arg(
      Arg::with_name("transfer")
        .long("transfer")
        .takes_value(true)
        .value_name("DEVICE")
        .help("Transfer playback to device"),
    )
    .arg(
      Arg::with_name("like")
        .long("like")
        .help("Like the current song"),
    )
    .arg(
      Arg::with_name("shuffle")
        .long("shuffle")
        .help("Toggle shuffle mode"),
    )
    .arg(
      Arg::with_name("repeat")
        .long("repeat")
        .help("Toggle repeat mode"),
    )
    .arg(
      Arg::with_name("next")
        .short("n")
        .long("next")
        .help("Jump to next song"),
    )
    .arg(
      Arg::with_name("previous")
        .short("p")
        .long("previous")
        .help("Jump to previous song"),
    )
    .arg(
      Arg::with_name("volume")
        .short("v")
        .long("volume")
        .takes_value(true)
        .value_name("VOLUME")
        .help("Turn volume up or down"),
    )
    .group(
      ArgGroup::with_name("jumps")
        .args(&["next", "previous"])
        .multiple(false),
    )
    .group(
      ArgGroup::with_name("flags")
        .args(&["like", "shuffle", "repeat"])
        .multiple(false),
    )
    .group(
      ArgGroup::with_name("actions")
        .args(&[
          "toggle", "status", "transfer", "like", "shuffle", "repeat", "next", "previous", "volume",
        ])
        .multiple(false)
        .required(true),
    )
}

pub fn play_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("play")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Play a uri or another spotify item by name")
    .visible_alias("p")
    .arg(device_arg())
    .arg(format_arg().default_value("%s %t - %a"))
    .arg(
      Arg::with_name("uri")
        .short("u")
        .long("uri")
        .takes_value(true)
        .value_name("URI")
        .help("Play the specified URI"),
    )
    .arg(
      Arg::with_name("name")
        .short("n")
        .long("name")
        .takes_value(true)
        .value_name("NAME")
        .help("Play the first match with NAME from specified category"),
    )
    .arg(
      Arg::with_name("queue")
        .short("q")
        .long("queue")
        // Only works with tracks
        .conflicts_with_all(&["album", "artist", "playlist", "show"])
        .help("Add track to queue instead of playing"),
    )
    .arg(
      Arg::with_name("album")
        .short("b")
        .long("album")
        .help("Look for albums"),
    )
    .arg(
      Arg::with_name("artist")
        .short("a")
        .long("artist")
        .help("Look for artists"),
    )
    .arg(
      Arg::with_name("track")
        .short("t")
        .long("track")
        .help("Look for tracks"),
    )
    .arg(
      Arg::with_name("show")
        .short("w")
        .long("show")
        .help("Look for shows"),
    )
    .arg(
      Arg::with_name("playlist")
        .short("p")
        .long("playlist")
        .help("Look for playlists"),
    )
    .group(
      ArgGroup::with_name("contexts")
        .args(&["track", "artist", "playlist", "album", "show"])
        .multiple(false),
    )
    .group(
      ArgGroup::with_name("actions")
        .args(&["uri", "name"])
        .multiple(false)
        .required(true),
    )
}

pub fn query_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("query")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Search for tracks, playlists and more")
    .visible_alias("q")
    .arg(format_arg().default_value_ifs(&[
      ("devices", None, "%v% %d"),
      ("liked", None, "%t - %a (%u)"),
      ("tracks", None, "%t - %a (%u)"),
      ("playlists", None, "%p (%u)"),
      ("artists", None, "%a (%u)"),
      ("albums", None, "%l - %(%u)"),
      ("shows", None, "%h - %(%u)"),
      // These have to be at the end because clap just takes the first match
      // '--list' defaults to devices
      ("list", None, "%v% %d"),
      // '--search' defaults to tracks
      ("search", None, "%t - %a (%u)"),
    ]))
    // Listing
    .arg(
      Arg::with_name("list")
        .short("l")
        .long("list")
        .help("List devices and playlists"),
    )
    .arg(
      Arg::with_name("devices")
        .short("d")
        .long("devices")
        .help("List the category 'devices'"),
    )
    .arg(
      Arg::with_name("playlists")
        .short("p")
        .long("playlists")
        .help("List / search the category 'playlists'"),
    )
    .arg(
      Arg::with_name("liked")
        .long("liked")
        .help("List the category 'liked songs'"),
    )
    .group(
      ArgGroup::with_name("listable")
        .args(&["devices", "playlists", "liked"])
        .multiple(false),
    )
    // Searching
    .arg(
      Arg::with_name("search")
        .short("s")
        .long("search")
        .takes_value(true)
        .value_name("SEARCH")
        .help("Search for tracks etc."),
    )
    .arg(
      Arg::with_name("albums")
        .short("b")
        .long("albums")
        .help("Search the category 'albums'"),
    )
    .arg(
      Arg::with_name("artists")
        .short("a")
        .long("artists")
        .help("Search the category 'artists'"),
    )
    .arg(
      Arg::with_name("tracks")
        .short("t")
        .long("tracks")
        .help("Search the category 'tracks'"),
    )
    .arg(
      Arg::with_name("shows")
        .short("w")
        .long("shows")
        .help("Search the category 'shows'"),
    )
    .arg(
      Arg::with_name("limit")
        .long("limit")
        .takes_value(true)
        .help("Specify the max number of results"),
    )
    .group(
      ArgGroup::with_name("searchable")
        .args(&["playlists", "tracks", "albums", "artists", "shows"])
        .multiple(false),
    )
    .group(
      ArgGroup::with_name("actions")
        .args(&["list", "search"])
        .multiple(false)
        .required(true),
    )
}
