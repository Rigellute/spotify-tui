# Spotify TUI

![Continuous Integration](https://github.com/Rigellute/spotify-tui/workflows/Continuous%20Integration/badge.svg?branch=master&event=push)
![](https://img.shields.io/badge/license-MIT-blueviolet.svg)
![](https://tokei.rs/b1/github/Rigellute/spotify-tui?category=code)
[![Crates.io](https://img.shields.io/crates/v/spotify-tui.svg)](https://crates.io/crates/spotify-tui)
![](https://img.shields.io/github/v/release/Rigellute/spotify-tui?color=%23c694ff)

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-81-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

[![Follow Alexander Keliris (Rigellute)](https://img.shields.io/twitter/follow/AlexKeliris?label=Follow%20Alexander%20Keliris%20%28Rigellute%29&style=social)](https://twitter.com/intent/follow?screen_name=AlexKeliris)

A Spotify client for the terminal written in Rust.

![Demo](https://user-images.githubusercontent.com/12150276/75177190-91d4ab00-572d-11ea-80bd-c5e28c7b17ad.gif)

The terminal in the demo above is using the [Rigel theme](https://rigel.netlify.com/).

- [Spotify TUI](#spotify-tui)
  - [Installation](#installation)
    - [Homebrew](#homebrew)
    - [Snap](#snap)
    - [AUR](#aur)
    - [Nix](#nix)
    - [Void Linux](#void-linux)
    - [Fedora/CentOS](#fedora-centos)
    - [Cargo](#cargo)
      - [Note on Linux](#note-on-linux)
    - [Windows](#windows-10)
      - [Scoop installer](#scoop-installer)
    - [Manual](#manual)
  - [Connecting to Spotify’s API](#connecting-to-spotifys-api)
  - [Usage](#usage)
- [Configuration](#configuration)
  - [Limitations](#limitations)
  - [Using with spotifyd](#using-with-spotifyd)
  - [Libraries used](#libraries-used)
  - [Development](#development)
    - [Windows Subsystem for Linux](#windows-subsystem-for-linux)
  - [Contributors](#contributors)
  - [Roadmap](#roadmap)
    - [High-level requirements yet to be implemented](#high-level-requirements-yet-to-be-implemented)

## Installation

The binary executable is `spt`.

### Homebrew

For both macOS and Linux

```bash
brew install spotify-tui
```

To update, run

```bash
brew upgrade spotify-tui
```

### Snap

For a system with Snap installed, run

```bash
snap install spt
```

The stable version will be installed for you automatically.

If you want to install the nightly build, run

```bash
snap install spt --edge
```

### AUR

For those on Arch Linux you can find the package on AUR [here](https://aur.archlinux.org/packages/spotify-tui/). If however you're using an AUR helper you can install directly from that, for example (in the case of [yay](https://github.com/Jguer/yay)), run

```bash
yay -S spotify-tui
```

### Nix

Available as the package `spotify-tui`. To install run:

```bash
nix-env -iA nixpkgs.spotify-tui
```

Where `nixpkgs` is the channel name in your configuration. For a more up-to-date installation, use the unstable channel.
It is also possible to add the package to `environment.systemPackages` (for NixOS), or `home.packages` when using [home-manager](https://github.com/rycee/home-manager).

### Void Linux

Available on the official repositories. To install, run

```bash
sudo xbps-install -Su spotify-tui
```

### Fedora/CentOS

Available on the [Copr](https://copr.fedorainfracloud.org/coprs/atim/spotify-tui/) repositories. To install, run

```bash
sudo dnf copr enable atim/spotify-tui -y && sudo dnf install spotify-tui
```

### Cargo

Use this option if your architecture is not supported by the pre-built binaries found on the [releases page](https://github.com/Rigellute/spotify-tui/releases).

First, install [Rust](https://www.rust-lang.org/tools/install) (using the recommended `rustup` installation method) and then

```bash
cargo install spotify-tui
```

This method will build the binary from source.

To update, run the same command again.

#### Note on Linux

For compilation on Linux the development packages for `libssl` are required.
For basic installation instructions, see [install OpenSSL](https://docs.rs/openssl/0.10.25/openssl/#automatic).
In order to locate dependencies, the compilation also requires `pkg-config` to be installed.

If you are using the Windows Subsystem for Linux, you'll need to [install additional dependencies](#windows-subsystem-for-linux).

### Windows 10

#### Scoop installer

First, make sure scoop installer is on your windows box, for instruction please visit [scoop.sh](https://scoop.sh)

Then open powershell and run following two commands:

```bash
scoop bucket add scoop-bucket https://github.com/Rigellute/scoop-bucket
scoop install spotify-tui
```

After that program is available as: `spt` or `spt.exe`

### Manual

1. Download the latest [binary](https://github.com/Rigellute/spotify-tui/releases) for your OS.
1. `cd` to the file you just downloaded and unzip
1. `cd` to `spotify-tui` and run with `./spt`

## Connecting to Spotify’s API

`spotify-tui` needs to connect to Spotify’s API in order to find music by
name, play tracks etc.

Instructions on how to set this up will be shown when you first run the app.

But here they are again:

1. Go to the [Spotify dashboard](https://developer.spotify.com/dashboard/applications)
1. Click `Create an app`
    - You now can see your `Client ID` and `Client Secret`
1. Now click `Edit Settings`
1. Add `http://localhost:8888/callback` to the Redirect URIs
1. Scroll down and click `Save`
1. You are now ready to authenticate with Spotify!
1. Go back to the terminal
1. Run `spt`
1. Enter your `Client ID`
1. Enter your `Client Secret`
1. Press enter to confirm the default port (8888) or enter a custom port
1. You will be redirected to an official Spotify webpage to ask you for permissions.
1. After accepting the permissions, you'll be redirected to localhost. If all goes well, the redirect URL will be parsed automatically and now you're done. If the local webserver fails for some reason you'll be redirected to a blank webpage that might say something like "Connection Refused" since no server is running. Regardless, copy the URL and paste into the prompt in the terminal.

And now you are ready to use the `spotify-tui` 🎉

You can edit the config at anytime at `${HOME}/.config/spotify-tui/client.yml`.

## Usage

The binary is named `spt`.

Running `spt` with no arguments will bring up the UI. Press `?` to bring up a help menu that shows currently implemented key events and their actions.
There is also a CLI that is able to do most of the stuff the UI does. Use `spt --help` to learn more.

Here are some example to get you excited.
```
spt --completions zsh # Prints shell completions for zsh to stdout (bash, power-shell and more are supported)

spt play --name "Your Playlist" --playlist --random # Plays a random song from "Your Playlist"
spt play --name "A cool song" --track # Plays 'A cool song'

spt playback --like --shuffle # Likes the current song and toggles shuffle mode
spt playback --toggle # Plays/pauses the current playback

spt list --liked --limit 50 # See your liked songs (50 is the max limit)

# Looks for 'An even cooler song' and gives you the '{name} from {album}' of up to 30 matches
spt search "An even cooler song" --tracks --format "%t from %b" --limit 30
```

# Configuration

A configuration file is located at `${HOME}/.config/spotify-tui/config.yml`, for snap `${HOME}/snap/spt/current/.config/spotify-tui/config.yml`
(not to be confused with client.yml which handles spotify authentication)

The following is a sample config.yml file:

```yaml
# Sample config file

# The theme colours can be an rgb string of the form "255, 255, 255" or a string that references the colours from your terminal theme: Reset, Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray, DarkGray, LightRed, LightGreen, LightYellow, LightBlue, LightMagenta, LightCyan, White.
theme:
  active: Cyan # current playing song in list
  banner: LightCyan # the "spotify-tui" banner on launch
  error_border: Red # error dialog border
  error_text: LightRed # error message text (e.g. "Spotify API reported error 404")
  hint: Yellow # hint text in errors
  hovered: Magenta # hovered pane border
  inactive: Gray # borders of inactive panes
  playbar_background: Black # background of progress bar
  playbar_progress: LightCyan # filled-in part of the progress bar
  playbar_progress_text: Cyan # song length and time played/left indicator in the progress bar
  playbar_text: White # artist name in player pane
  selected: LightCyan # a) selected pane border, b) hovered item in list, & c) track title in player
  text: "255, 255, 255" # text in panes
  header: White # header text in panes (e.g. 'Title', 'Artist', etc.)

behavior:
  seek_milliseconds: 5000
  volume_increment: 10
  # The lower the number the higher the "frames per second". You can decrease this number so that the audio visualisation is smoother but this can be expensive!
  tick_rate_milliseconds: 250
  # Enable text emphasis (typically italic/bold text styling). Disabling this might be important if the terminal config is otherwise restricted and rendering text escapes interferes with the UI.
  enable_text_emphasis: true
  # Controls whether to show a loading indicator in the top right of the UI whenever communicating with Spotify API
  show_loading_indicator: true
  # Determines the text icon to display next to "liked" Spotify items, such as
  # liked songs and albums, or followed artists. Can be any length string.
  # These icons require a patched nerd font.
  liked_icon: ♥
  shuffle_icon: 🔀
  repeat_track_icon: 🔂
  repeat_context_icon: 🔁
  playing_icon: ▶
  paused_icon: ⏸

keybindings:
  # Key stroke can be used if it only uses two keys:
  # ctrl-q works,
  # ctrl-alt-q doesn't.
  back: "ctrl-q"

  jump_to_album: "a"

  # Shift modifiers use a capital letter (also applies with other modifier keys
  # like ctrl-A)
  jump_to_artist_album: "A"

  manage_devices: "d"
  decrease_volume: "-"
  increase_volume: "+"
  toggle_playback: " "
  seek_backwards: "<"
  seek_forwards: ">"
  next_track: "n"
  previous_track: "p"
  copy_song_url: "c"
  copy_album_url: "C"
  help: "?"
  shuffle: "s"
  repeat: "r"
  search: "/"
  audio_analysis: "v"
  jump_to_context: "o"
  basic_view: "B"
  add_item_to_queue: "z"
```

## Limitations

This app uses the [Web API](https://developer.spotify.com/documentation/web-api/) from Spotify, which doesn't handle streaming itself. So you'll need either an official Spotify client open or a lighter weight alternative such as [spotifyd](https://github.com/Spotifyd/spotifyd).

If you want to play tracks, Spotify requires that you have a Premium account.

## Using with [spotifyd](https://github.com/Spotifyd/spotifyd)

Follow the spotifyd documentation to get set up.

After that there is not much to it.

1. Start running the spotifyd daemon.
1. Start up `spt`
1. Press `d` to go to the device selection menu and the spotifyd "device" should be there - if not check [these docs](https://github.com/Spotifyd/spotifyd#logging)

## Libraries used

- [tui-rs](https://github.com/fdehau/tui-rs)
- [rspotify](https://github.com/ramsayleung/rspotify)

## Development

1. [Install OpenSSL](https://docs.rs/openssl/0.10.25/openssl/#automatic)
1. [Install Rust](https://www.rust-lang.org/tools/install)
1. [Install `xorg-dev`](https://github.com/aweinstock314/rust-clipboard#prerequisites) (required for clipboard support)
1. Clone or fork this repo and `cd` to it
1. And then `cargo run`

### Windows Subsystem for Linux

You might get a linking error. If so, you'll probably need to install additional dependencies required by the clipboard package

```bash
sudo apt-get install -y -qq pkg-config libssl-dev libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

## Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://keliris.dev/"><img src="https://avatars2.githubusercontent.com/u/12150276?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Alexander Keliris</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=Rigellute" title="Code">💻</a> <a href="https://github.com/Rigellute/spotify-tui/commits?author=Rigellute" title="Documentation">📖</a> <a href="#design-Rigellute" title="Design">🎨</a> <a href="#blog-Rigellute" title="Blogposts">📝</a> <a href="#ideas-Rigellute" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-Rigellute" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-Rigellute" title="Maintenance">🚧</a> <a href="#platform-Rigellute" title="Packaging/porting to new platform">📦</a> <a href="https://github.com/Rigellute/spotify-tui/pulls?q=is%3Apr+reviewed-by%3ARigellute" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://github.com/mikepombal"><img src="https://avatars3.githubusercontent.com/u/6864231?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Mickael Marques</b></sub></a><br /><a href="#financial-mikepombal" title="Financial">💵</a></td>
    <td align="center"><a href="https://github.com/HakierGrzonzo"><img src="https://avatars0.githubusercontent.com/u/36668331?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Grzegorz Koperwas</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=HakierGrzonzo" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/amgassert"><img src="https://avatars2.githubusercontent.com/u/22896005?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Austin Gassert</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=amgassert" title="Code">💻</a></td>
    <td align="center"><a href="https://robinette.dev"><img src="https://avatars2.githubusercontent.com/u/30757528?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Calen Robinette</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=calenrobinette" title="Code">💻</a></td>
    <td align="center"><a href="https://mcofficer.me"><img src="https://avatars0.githubusercontent.com/u/22377202?v=4?s=100" width="100px;" alt=""/><br /><sub><b>M*C*O</b></sub></a><br /><a href="#infra-MCOfficer" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a></td>
    <td align="center"><a href="https://github.com/eminence"><img src="https://avatars0.githubusercontent.com/u/402454?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Andrew Chin</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=eminence" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://www.samnaser.com/"><img src="https://avatars0.githubusercontent.com/u/4377348?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Sam Naser</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=Monkeyanator" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/radogost"><img src="https://avatars0.githubusercontent.com/u/15713820?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Micha</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=radogost" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/neriglissar"><img src="https://avatars2.githubusercontent.com/u/53038761?v=4?s=100" width="100px;" alt=""/><br /><sub><b>neriglissar</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=neriglissar" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/TimonPost"><img src="https://avatars3.githubusercontent.com/u/19969910?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Timon</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=TimonPost" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/echoSayonara"><img src="https://avatars2.githubusercontent.com/u/54503126?v=4?s=100" width="100px;" alt=""/><br /><sub><b>echoSayonara</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=echoSayonara" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/D-Nice"><img src="https://avatars1.githubusercontent.com/u/2888248?v=4?s=100" width="100px;" alt=""/><br /><sub><b>D-Nice</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=D-Nice" title="Documentation">📖</a> <a href="#infra-D-Nice" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a></td>
    <td align="center"><a href="http://gpawlik.com"><img src="https://avatars3.githubusercontent.com/u/6296883?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Grzegorz Pawlik</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=gpawlik" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="http://lenny.ninja"><img src="https://avatars1.githubusercontent.com/u/4027243?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Lennart Bernhardt</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=LennyPenny" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/BlackYoup"><img src="https://avatars3.githubusercontent.com/u/6098160?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Arnaud Lefebvre</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=BlackYoup" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/tem1029"><img src="https://avatars3.githubusercontent.com/u/57712713?v=4?s=100" width="100px;" alt=""/><br /><sub><b>tem1029</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=tem1029" title="Code">💻</a></td>
    <td align="center"><a href="http://peter.moss.dk"><img src="https://avatars2.githubusercontent.com/u/12544579?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Peter K. Moss</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=Peterkmoss" title="Code">💻</a></td>
    <td align="center"><a href="http://www.zephyrizing.net/"><img src="https://avatars1.githubusercontent.com/u/113102?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Geoff Shannon</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=RadicalZephyr" title="Code">💻</a></td>
    <td align="center"><a href="http://zacklukem.info"><img src="https://avatars0.githubusercontent.com/u/8787486?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Zachary Mayhew</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=zacklukem" title="Code">💻</a></td>
    <td align="center"><a href="http://jfaltis.de"><img src="https://avatars2.githubusercontent.com/u/45465572?v=4?s=100" width="100px;" alt=""/><br /><sub><b>jfaltis</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=jfaltis" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://marcelschr.me"><img src="https://avatars3.githubusercontent.com/u/19377618?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Marcel Schramm</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=Bios-Marcel" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/fangyi-zhou"><img src="https://avatars3.githubusercontent.com/u/7815439?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Fangyi Zhou</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=fangyi-zhou" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/synth-ruiner"><img src="https://avatars1.githubusercontent.com/u/8642013?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Max</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=synth-ruiner" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/svenvNL"><img src="https://avatars1.githubusercontent.com/u/13982006?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Sven van der Vlist</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=svenvNL" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jacobchrismarsh"><img src="https://avatars2.githubusercontent.com/u/15932179?v=4?s=100" width="100px;" alt=""/><br /><sub><b>jacobchrismarsh</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=jacobchrismarsh" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/TheWalkingLeek"><img src="https://avatars2.githubusercontent.com/u/36076343?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Nils Rauch</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=TheWalkingLeek" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/sputnick1124"><img src="https://avatars1.githubusercontent.com/u/8843309?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Nick Stockton</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=sputnick1124" title="Code">💻</a> <a href="https://github.com/Rigellute/spotify-tui/issues?q=author%3Asputnick1124" title="Bug reports">🐛</a> <a href="#maintenance-sputnick1124" title="Maintenance">🚧</a> <a href="#question-sputnick1124" title="Answering Questions">💬</a> <a href="https://github.com/Rigellute/spotify-tui/commits?author=sputnick1124" title="Documentation">📖</a></td>
  </tr>
  <tr>
    <td align="center"><a href="http://stuarth.github.io"><img src="https://avatars3.githubusercontent.com/u/7055?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Stuart Hinson</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=stuarth" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/samcal"><img src="https://avatars3.githubusercontent.com/u/2117940?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Sam Calvert</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=samcal" title="Code">💻</a> <a href="https://github.com/Rigellute/spotify-tui/commits?author=samcal" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/jwijenbergh"><img src="https://avatars0.githubusercontent.com/u/46386452?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Jeroen Wijenbergh</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=jwijenbergh" title="Documentation">📖</a></td>
    <td align="center"><a href="https://twitter.com/KimberleyCook91"><img src="https://avatars3.githubusercontent.com/u/2683270?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Kimberley Cook</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=KimberleyCook" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/baxtea"><img src="https://avatars0.githubusercontent.com/u/22502477?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Audrey Baxter</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=baxtea" title="Code">💻</a></td>
    <td align="center"><a href="https://koehr.in"><img src="https://avatars2.githubusercontent.com/u/246402?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Norman</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=nkoehring" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/blackwolf12333"><img src="https://avatars0.githubusercontent.com/u/1572975?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Peter Maatman</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=blackwolf12333" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/AlexandreSi"><img src="https://avatars1.githubusercontent.com/u/32449369?v=4?s=100" width="100px;" alt=""/><br /><sub><b>AlexandreS</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=AlexandreSi" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/fiinnnn"><img src="https://avatars2.githubusercontent.com/u/5011796?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Finn Vos</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=fiinnnn" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/hurricanehrndz"><img src="https://avatars0.githubusercontent.com/u/5804237?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Carlos Hernandez</b></sub></a><br /><a href="#platform-hurricanehrndz" title="Packaging/porting to new platform">📦</a></td>
    <td align="center"><a href="https://github.com/pedrohva"><img src="https://avatars3.githubusercontent.com/u/33297928?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Pedro Alves</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=pedrohva" title="Code">💻</a></td>
    <td align="center"><a href="https://gitlab.com/jtagcat/"><img src="https://avatars1.githubusercontent.com/u/38327267?v=4?s=100" width="100px;" alt=""/><br /><sub><b>jtagcat</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=jtagcat" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/BKitor"><img src="https://avatars0.githubusercontent.com/u/16880850?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Benjamin Kitor</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=BKitor" title="Code">💻</a></td>
    <td align="center"><a href="https://ales.rocks"><img src="https://avatars0.githubusercontent.com/u/544082?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Aleš Najmann</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=littleli" title="Documentation">📖</a> <a href="#platform-littleli" title="Packaging/porting to new platform">📦</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/jeremystucki"><img src="https://avatars3.githubusercontent.com/u/7629727?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Jeremy Stucki</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=jeremystucki" title="Code">💻</a></td>
    <td align="center"><a href="http://pt2121.github.io"><img src="https://avatars0.githubusercontent.com/u/616399?v=4?s=100" width="100px;" alt=""/><br /><sub><b>(´⌣`ʃƪ)</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=pt2121" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/tim77"><img src="https://avatars0.githubusercontent.com/u/5614476?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Artem Polishchuk</b></sub></a><br /><a href="#platform-tim77" title="Packaging/porting to new platform">📦</a></td>
    <td align="center"><a href="https://github.com/slumber"><img src="https://avatars2.githubusercontent.com/u/48099298?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Chris Sosnin</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=slumber" title="Code">💻</a></td>
    <td align="center"><a href="http://www.benbuhse.com"><img src="https://avatars1.githubusercontent.com/u/21225303?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Ben Buhse</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=bwbuhse" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/ilnaes"><img src="https://avatars1.githubusercontent.com/u/20805499?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Sean Li</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=ilnaes" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/TimotheeGerber"><img src="https://avatars3.githubusercontent.com/u/37541513?v=4?s=100" width="100px;" alt=""/><br /><sub><b>TimotheeGerber</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=TimotheeGerber" title="Code">💻</a> <a href="https://github.com/Rigellute/spotify-tui/commits?author=TimotheeGerber" title="Documentation">📖</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/fratajczak"><img src="https://avatars2.githubusercontent.com/u/33835579?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Ferdinand Ratajczak</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=fratajczak" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/sheelc"><img src="https://avatars0.githubusercontent.com/u/1355710?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Sheel Choksi</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=sheelc" title="Code">💻</a></td>
    <td align="center"><a href="http://fnanp.in-ulm.de/microblog/"><img src="https://avatars1.githubusercontent.com/u/414112?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Michael Hellwig</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=mhellwig" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/oliver-daniel"><img src="https://avatars2.githubusercontent.com/u/17235417?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Oliver Daniel</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=oliver-daniel" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Drewsapple"><img src="https://avatars2.githubusercontent.com/u/4532572?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Drew Fisher</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=Drewsapple" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/ncoder-1"><img src="https://avatars0.githubusercontent.com/u/7622286?v=4?s=100" width="100px;" alt=""/><br /><sub><b>ncoder-1</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=ncoder-1" title="Documentation">📖</a></td>
    <td align="center"><a href="http://macguire.me"><img src="https://avatars3.githubusercontent.com/u/18323154?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Macguire Rintoul</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=macguirerintoul" title="Documentation">📖</a></td>
  </tr>
  <tr>
    <td align="center"><a href="http://ricardohe97.github.io"><img src="https://avatars3.githubusercontent.com/u/28399979?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Ricardo Holguin</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=RicardoHE97" title="Code">💻</a></td>
    <td align="center"><a href="https://ksk.netlify.com"><img src="https://avatars3.githubusercontent.com/u/13160198?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Keisuke Toyota</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=ksk001100" title="Code">💻</a></td>
    <td align="center"><a href="https://jackson15j.github.io"><img src="https://avatars1.githubusercontent.com/u/3226988?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Craig Astill</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=jackson15j" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/onielfa"><img src="https://avatars0.githubusercontent.com/u/4358172?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Onielfa</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=onielfa" title="Code">💻</a></td>
    <td align="center"><a href="https://usrme.xyz"><img src="https://avatars3.githubusercontent.com/u/5902545?v=4?s=100" width="100px;" alt=""/><br /><sub><b>usrme</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=usrme" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/murlakatamenka"><img src="https://avatars2.githubusercontent.com/u/7361274?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Sergey A.</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=murlakatamenka" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/elcih17"><img src="https://avatars3.githubusercontent.com/u/17084445?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Hideyuki Okada</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=elcih17" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/kepae"><img src="https://avatars2.githubusercontent.com/u/4238598?v=4?s=100" width="100px;" alt=""/><br /><sub><b>kepae</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=kepae" title="Code">💻</a> <a href="https://github.com/Rigellute/spotify-tui/commits?author=kepae" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/ericonr"><img src="https://avatars0.githubusercontent.com/u/34201958?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Érico Nogueira Rolim</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=ericonr" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/BeneCollyridam"><img src="https://avatars2.githubusercontent.com/u/15802915?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Alexander Meinhardt Scheurer</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=BeneCollyridam" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Toaster192"><img src="https://avatars0.githubusercontent.com/u/14369229?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Ondřej Kinšt</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=Toaster192" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Kryan90"><img src="https://avatars3.githubusercontent.com/u/18740821?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Kryan90</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=Kryan90" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/n-ivanov"><img src="https://avatars3.githubusercontent.com/u/11470871?v=4?s=100" width="100px;" alt=""/><br /><sub><b>n-ivanov</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=n-ivanov" title="Code">💻</a></td>
    <td align="center"><a href="http://matthewbilyeu.com/resume/"><img src="https://avatars3.githubusercontent.com/u/1185129?v=4?s=100" width="100px;" alt=""/><br /><sub><b>bi1yeu</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=bi1yeu" title="Code">💻</a> <a href="https://github.com/Rigellute/spotify-tui/commits?author=bi1yeu" title="Documentation">📖</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/Utagai"><img src="https://avatars2.githubusercontent.com/u/10730394?v=4?s=100" width="100px;" alt=""/><br /><sub><b>May</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=Utagai" title="Code">💻</a></td>
    <td align="center"><a href="https://mucinoab.github.io/"><img src="https://avatars1.githubusercontent.com/u/28630268?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Bruno A. Muciño</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=mucinoab" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/OrangeFran"><img src="https://avatars2.githubusercontent.com/u/55061632?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Finn Hediger</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=OrangeFran" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/dp304"><img src="https://avatars1.githubusercontent.com/u/34493835?v=4?s=100" width="100px;" alt=""/><br /><sub><b>dp304</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=dp304" title="Code">💻</a></td>
    <td align="center"><a href="http://marcomicera.github.io"><img src="https://avatars0.githubusercontent.com/u/13918587?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Marco Micera</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=marcomicera" title="Documentation">📖</a></td>
    <td align="center"><a href="http://marcoieni.com"><img src="https://avatars3.githubusercontent.com/u/11428655?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Marco Ieni</b></sub></a><br /><a href="#infra-MarcoIeni" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a></td>
    <td align="center"><a href="https://github.com/ArturKovacs"><img src="https://avatars3.githubusercontent.com/u/8320264?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Artúr Kovács</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=ArturKovacs" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/aokellermann"><img src="https://avatars.githubusercontent.com/u/26678747?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Antony Kellermann</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=aokellermann" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rasmuspeders1"><img src="https://avatars.githubusercontent.com/u/1898960?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Rasmus Pedersen</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=rasmuspeders1" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/noir-Z"><img src="https://avatars.githubusercontent.com/u/45096516?v=4?s=100" width="100px;" alt=""/><br /><sub><b>noir-Z</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=noir-Z" title="Documentation">📖</a></td>
    <td align="center"><a href="https://davidbailey.codes/"><img src="https://avatars.githubusercontent.com/u/4248177?v=4?s=100" width="100px;" alt=""/><br /><sub><b>David Bailey</b></sub></a><br /><a href="https://github.com/Rigellute/spotify-tui/commits?author=davidbailey00" title="Documentation">📖</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

## Roadmap

The goal is to eventually implement almost every Spotify feature.

### High-level requirements yet to be implemented

- Add songs to a playlist
- Be able to scroll through result pages in every view

This table shows all that is possible with the Spotify API, what is implemented already, and whether that is essential.

| API method                                        | Implemented yet? | Explanation                                                                                                                                                  | Essential? |
| ------------------------------------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------- |
| track                                             | No               | returns a single track given the track's ID, URI or URL                                                                                                      | No         |
| tracks                                            | No               | returns a list of tracks given a list of track IDs, URIs, or URLs                                                                                            | No         |
| artist                                            | No               | returns a single artist given the artist's ID, URI or URL                                                                                                    | Yes        |
| artists                                           | No               | returns a list of artists given the artist IDs, URIs, or URLs                                                                                                | No         |
| artist_albums                                     | Yes              | Get Spotify catalog information about an artist's albums                                                                                                     | Yes        |
| artist_top_tracks                                 | Yes              | Get Spotify catalog information about an artist's top 10 tracks by country.                                                                                  | Yes        |
| artist_related_artists                            | Yes              | Get Spotify catalog information about artists similar to an identified artist. Similarity is based on analysis of the Spotify community's listening history. | Yes        |
| album                                             | Yes              | returns a single album given the album's ID, URIs or URL                                                                                                     | Yes        |
| albums                                            | No               | returns a list of albums given the album IDs, URIs, or URLs                                                                                                  | No         |
| search_album                                      | Yes              | Search album based on query                                                                                                                                  | Yes        |
| search_artist                                     | Yes              | Search artist based on query                                                                                                                                 | Yes        |
| search_track                                      | Yes              | Search track based on query                                                                                                                                  | Yes        |
| search_playlist                                   | Yes              | Search playlist based on query                                                                                                                               | Yes        |
| album_track                                       | Yes              | Get Spotify catalog information about an album's tracks                                                                                                      | Yes        |
| user                                              | No               | Gets basic profile information about a Spotify User                                                                                                          | No         |
| playlist                                          | Yes              | Get full details about Spotify playlist                                                                                                                      | Yes        |
| current_user_playlists                            | Yes              | Get current user playlists without required getting his profile                                                                                              | Yes        |
| user_playlists                                    | No               | Gets playlists of a user                                                                                                                                     | No         |
| user_playlist                                     | No               | Gets playlist of a user                                                                                                                                      | No         |
| user_playlist_tracks                              | Yes              | Get full details of the tracks of a playlist owned by a user                                                                                                 | Yes        |
| user_playlist_create                              | No               | Creates a playlist for a user                                                                                                                                | Yes        |
| user_playlist_change_detail                       | No               | Changes a playlist's name and/or public/private state                                                                                                        | Yes        |
| user_playlist_unfollow                            | Yes              | Unfollows (deletes) a playlist for a user                                                                                                                    | Yes        |
| user_playlist_add_track                           | No               | Adds tracks to a playlist                                                                                                                                    | Yes        |
| user_playlist_replace_track                       | No               | Replace all tracks in a playlist                                                                                                                             | No         |
| user_playlist_recorder_tracks                     | No               | Reorder tracks in a playlist                                                                                                                                 | No         |
| user_playlist_remove_all_occurrences_of_track     | No               | Removes all occurrences of the given tracks from the given playlist                                                                                          | No         |
| user_playlist_remove_specific_occurrenes_of_track | No               | Removes all occurrences of the given tracks from the given playlist                                                                                          | No         |
| user_playlist_follow_playlist                     | Yes              | Add the current authenticated user as a follower of a playlist.                                                                                              | Yes        |
| user_playlist_check_follow                        | No               | Check to see if the given users are following the given playlist                                                                                             | Yes        |
| me                                                | No               | Get detailed profile information about the current user.                                                                                                     | Yes        |
| current_user                                      | No               | Alias for `me`                                                                                                                                               | Yes        |
| current_user_playing_track                        | Yes              | Get information about the current users currently playing track.                                                                                             | Yes        |
| current_user_saved_albums                         | Yes              | Gets a list of the albums saved in the current authorized user's "Your Music" library                                                                        | Yes        |
| current_user_saved_tracks                         | Yes              | Gets the user's saved tracks or "Liked Songs"                                                                                                                | Yes        |
| current_user_followed_artists                     | Yes              | Gets a list of the artists followed by the current authorized user                                                                                           | Yes        |
| current_user_saved_tracks_delete                  | Yes              | Remove one or more tracks from the current user's "Your Music" library.                                                                                      | Yes        |
| current_user_saved_tracks_contain                 | No               | Check if one or more tracks is already saved in the current Spotify user’s “Your Music” library.                                                             | Yes        |
| current_user_saved_tracks_add                     | Yes              | Save one or more tracks to the current user's "Your Music" library.                                                                                          | Yes        |
| current_user_top_artists                          | No               | Get the current user's top artists                                                                                                                           | Yes        |
| current_user_top_tracks                           | No               | Get the current user's top tracks                                                                                                                            | Yes        |
| current_user_recently_played                      | Yes              | Get the current user's recently played tracks                                                                                                                | Yes        |
| current_user_saved_albums_add                     | Yes              | Add one or more albums to the current user's "Your Music" library.                                                                                           | Yes        |
| current_user_saved_albums_delete                  | Yes              | Remove one or more albums from the current user's "Your Music" library.                                                                                      | Yes        |
| user_follow_artists                               | Yes              | Follow one or more artists                                                                                                                                   | Yes        |
| user_unfollow_artists                             | Yes              | Unfollow one or more artists                                                                                                                                 | Yes        |
| user_follow_users                                 | No               | Follow one or more users                                                                                                                                     | No         |
| user_unfollow_users                               | No               | Unfollow one or more users                                                                                                                                   | No         |
| featured_playlists                                | No               | Get a list of Spotify featured playlists                                                                                                                     | Yes        |
| new_releases                                      | No               | Get a list of new album releases featured in Spotify                                                                                                         | Yes        |
| categories                                        | No               | Get a list of categories used to tag items in Spotify                                                                                                        | Yes        |
| recommendations                                   | Yes              | Get Recommendations Based on Seeds                                                                                                                           | Yes        |
| audio_features                                    | No               | Get audio features for a track                                                                                                                               | No         |
| audios_features                                   | No               | Get Audio Features for Several Tracks                                                                                                                        | No         |
| audio_analysis                                    | Yes              | Get Audio Analysis for a Track                                                                                                                               | Yes        |
| device                                            | Yes              | Get a User’s Available Devices                                                                                                                               | Yes        |
| current_playback                                  | Yes              | Get Information About The User’s Current Playback                                                                                                            | Yes        |
| current_playing                                   | No               | Get the User’s Currently Playing Track                                                                                                                       | No         |
| transfer_playback                                 | Yes              | Transfer a User’s Playback                                                                                                                                   | Yes        |
| start_playback                                    | Yes              | Start/Resume a User’s Playback                                                                                                                               | Yes        |
| pause_playback                                    | Yes              | Pause a User’s Playback                                                                                                                                      | Yes        |
| next_track                                        | Yes              | Skip User’s Playback To Next Track                                                                                                                           | Yes        |
| previous_track                                    | Yes              | Skip User’s Playback To Previous Track                                                                                                                       | Yes        |
| seek_track                                        | Yes              | Seek To Position In Currently Playing Track                                                                                                                  | Yes        |
| repeat                                            | Yes              | Set Repeat Mode On User’s Playback                                                                                                                           | Yes        |
| volume                                            | Yes              | Set Volume For User’s Playback                                                                                                                               | Yes        |
| shuffle                                           | Yes              | Toggle Shuffle For User’s Playback                                                                                                                           | Yes        |
