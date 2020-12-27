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
        .help("Returns the url to the current track")
        .conflicts_with("together"),
    )
    .arg(
      Arg::with_name("share-album")
        .long("share-album")
        .help("Returns the url to the current track")
        .conflicts_with("together"),
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
        .help("Jumps to the next song"),
    )
    .arg(
      Arg::with_name("previous")
        .short("p")
        .long("previous")
        .multiple(true)
        .help("Jumps to the previous song"),
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
        .multiple(false),
    )
    .group(
      ArgGroup::with_name("flags")
        .args(&["like", "shuffle", "repeat"])
        .multiple(true),
    )
    .group(
      ArgGroup::with_name("together")
        .args(&[
          "toggle", "status", "transfer", "like", "shuffle", "repeat", "next", "previous", "volume",
        ])
        .multiple(true)
        .conflicts_with("single"),
    )
    .group(
      ArgGroup::with_name("single")
        .args(&["share-track", "share-album"])
        .multiple(false)
        .conflicts_with("together"),
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
    .about("Lists devices, liked songs or searches for tracks, albums and more")
    .visible_alias("q")
    .arg(format_arg().default_value_ifs(&[
      ("devices", None, "%v% %d"),
      ("liked", None, "%t - %a (%u)"),
      ("tracks", None, "%t - %a (%u)"),
      ("playlists", None, "%p (%u)"),
      ("artists", None, "%a (%u)"),
      ("albums", None, "%b - %a (%u)"),
      ("shows", None, "%h - %a (%u)"),
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
        .help("Lists devices, playlists or liked songs"),
    )
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
        .help("Lists or looks for playlists"),
    )
    .arg(
      Arg::with_name("liked")
        .long("liked")
        .help("Lists liked songs"),
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
        .help("Looks for something on spotify"),
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
        .multiple(false),
    )
    .group(
      ArgGroup::with_name("actions")
        .args(&["list", "search"])
        .multiple(false)
        .required(true),
    )
}
