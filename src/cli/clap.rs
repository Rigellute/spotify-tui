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
    .help("Specifies the output format")
    .long_help(
      "There are multiple format specifiers you can use: %a: artist, %b: album, %p: playlist, \
%t: track, %h: show, %f: flags (shuffle, repeat, like), %s: playback status, %v: volume, %d: current device. \
Example: spt pb -s -f 'playing on %d at %v%'",
    )
}

pub fn playback_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("playback")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Interacts with the playback of a device")
    .long_about(
      "Use `playback` to interact with the playback of the current or any other device. \
You can specify another device with `--device`. If no options were provided, spt \
will default to just displaying the current playback. Actually, after every action \
spt will display the updated playback. The output format is configurable with the \
`--format` flag. Some options can be used together, other options have to be alone.

Here's a list:

* `--next` and `--previous` cannot be used with other options
* `--status`, `--toggle`, `--transfer`, `--volume`, `--like`, `--repeat` and `--shuffle` \
can be used together
* `--share-track` and `--share-album` cannot be used with other options",
    )
    .visible_alias("pb")
    .arg(device_arg())
    .arg(
      format_arg()
        .default_value("%f %s %t - %a")
        .default_value_ifs(&[
          ("seek", None, "%f %s %t - %a %r"),
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
        .help("Prints out the current status of a device (default)"),
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
        .help("Likes the current song if possible"),
    )
    .arg(
      Arg::with_name("dislike")
        .long("dislike")
        .help("Dislikes the current song if possible"),
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
        .help("Jumps to the next song")
        .long_help(
          "This jumps to the next song if specied once. If you want to jump, let's say 3 songs \
forward, you can use `--next` 3 times: `spt pb -nnn`.",
        ),
    )
    .arg(
      Arg::with_name("previous")
        .short("p")
        .long("previous")
        .multiple(true)
        .help("Jumps to the previous song")
        .long_help(
          "This jumps to the beginning of the current song if specied once. You probably want to \
jump to the previous song though, so you can use the previous flag twice: `spt pb -pp`. To jump \
two songs back, you can use `spt pb -ppp` and so on.",
        ),
    )
    .arg(
      Arg::with_name("seek")
        .long("seek")
        .takes_value(true)
        .value_name("±SECONDS")
        .allow_hyphen_values(true)
        .help("Jumps SECONDS forwards (+) or backwards (-)")
        .long_help(
          "For example: `spt pb --seek +10` jumps ten second forwards, `spt pb --seek -10` ten \
seconds backwards and `spt pb --seek 10` to the tenth second of the track.",
        ),
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
      ArgGroup::with_name("likes")
        .args(&["like", "dislike"])
        .multiple(false),
    )
    .group(
      ArgGroup::with_name("flags")
        .args(&["like", "dislike", "shuffle", "repeat"])
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
    .long_about(
      "If you specify a uri, the type can be inferred. If you want to play something by \
name, you have to specify the type: `--track`, `--album`, `--artist`, `--playlist` \
or `--show`. The first item which was found will be played without confirmation. \
To add a track to the queue, use `--queue`. To play a random song from a playlist, \
use `--random`. Again, with `--format` you can specify how the output will look. \
The same function as found in `playback` will be called.",
    )
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
        .requires("contexts")
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
    .about("Lists devices, liked songs and playlists")
    .long_about(
      "This will list devices, liked songs or playlists. With the `--limit` flag you are \
able to specify the amount of results (between 1 and 50). Here, the `--format` is \
even more awesome, get your output exactly the way you want. The format option will \
be applied to every item found.",
    )
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
    .long_about(
      "This will search for something on spotify and displays you the items. The output \
format can be changed with the `--format` flag and the limit can be changed with \
the `--limit` flag (between 1 and 50). The type can't be inferred, so you have to \
specify it.",
    )
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
