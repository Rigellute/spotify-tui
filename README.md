# Spotify TUI

![](https://github.com/Rigellute/spotify-tui/workflows/Continuous%20Integration/badge.svg)
![](https://img.shields.io/badge/license-MIT-blueviolet.svg)
![](https://tokei.rs/b1/github/Rigellute/spotify-tui?category=code)
[![Crates.io](https://img.shields.io/crates/v/spotify-tui.svg)](https://crates.io/crates/spotify-tui)
![](https://img.shields.io/github/v/release/Rigellute/spotify-tui?color=%23c694ff)

A Spotify client for the terminal written in Rust.

![Demo](https://user-images.githubusercontent.com/12150276/64545371-84af3580-d320-11e9-867d-c368fd888b3b.gif)

The terminal in the demo above is using the [Rigel theme](https://rigel.netlify.com/).

- [Installation](#installation)
- [Connecting to Spotify’s API](#connecting-to-spotifys-api)
- [Usage](#usage)
- [Limitations](#limitations)
- [Using with spotifyd](#using-with-spotifyd)
- [Development](#development)
- [Roadmap](#roadmap)

## Installation

The binary executable is `spt`.

### Homebrew

For both macOS and Linux

```bash
brew install Rigellute/tap/spotify-tui
```

To update, run

```bash
brew upgrade spotify-tui
```

### Cargo

Use this option if your architecture is not supported by the pre-built binaries found on the [releases page](https://github.com/Rigellute/spotify-tui/releases).

First, install [Rust](https://www.rust-lang.org/tools/install) (using the recommended `rustup` installation method) and then

```bash
cargo install spotify-tui
```

This method will build the binary from source.

To update, run

```bash
cargo install spotify-tui --force
```

#### Note on Linux

For compilation on Linux the development packages for `libssl` are required.
For basic installation instructions, see [install OpenSSL](https://docs.rs/openssl/0.10.25/openssl/#automatic).
In order to locate dependencies, the compilation also requires `pkg-config` to be installed.

### Manual

1. Download the latest [binary](https://github.com/Rigellute/spotify-tui/releases) for your OS.
1. `cd` to the file you just downloaded and unzip
1. `cd` to `spotify-ui` and run with `./spt`

### Note on Windows

The core dependancy `termion` [does not yet support Windows](https://gitlab.redox-os.org/redox-os/termion/issues/103), and so nor does `spotify-tui`. Apologies for this!

You can, however, run `spotify-tui` within the Windows Subsystem for Linux (which I have tested).

## Connecting to Spotify’s API

`spotify-tui` needs to connect to Spotify’s API in order to find music by
name, play tracks etc.

Instructions on how to set this up will be shown when you first run the app.

But here they are again:

1. Go to the [Spotify dashboard](https://developer.spotify.com/dashboard/applications)
1. Click `Create a Client ID` and create an app
1. Now click `Edit Settings`
1. Add `http://localhost:8888/callback` to the Redirect URIs
1. You are now ready to authenticate with Spotify!
1. Go back to the terminal
1. Run `spt`
1. Enter your `Client ID`
1. Enter your `Client Secret`
1. You will be redirected to an official Spotify webpage to ask you for permissions.
1. After accepting the permissions, you'll be redirected to localhost. If all goes well, the redirect URL will be parsed automatically and now you're done. If the local webserver fails for some reason you'll be redirected to a blank webpage that might say something like "Connection Refused" since no server is running. Regardless, copy the URL and paste into the prompt in the terminal.

And now you are ready to use the `spotify-tui` 🎉

You can edit the config at anytime at `${HOME}/.config/spotify-tui/client.yml`.

## Usage

The binary is named `spt`.

When running `spotify-tui` press `?` to bring up a help menu that shows currently implemented key events and their actions.

# Configuration

A configuration file is located at `${HOME}/.config/spotify-tui/config.yml`
(not to be confused with client.yml which handles spotify authentication)

The following is a sample config.yml file:
```yaml
    # Sample config file

keybindings:
    # Key stroke can be used if it only uses two keys:
    # ctrl-q works,
    # ctrl-alt-q doesn't.
    back: 'ctrl-q'

    jump_to_album: 'a'

    # Shift modifiers use a capital letter (also applies with other modifier keys
    # like ctrl-A)
    jump_to_artist_album: 'A'

    manage_devices: 'd'
    decrease_volume: '-'
    increase_volume: '+'
    toggle_playback: ' '
    seek_backwards: '<'
    seek_forwards: '>'
    next_track: 'n'
    previous_track: 'p'
    help: '?'
    shuffle: 's'
    repeat: 'r'
    search: '/'
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
1. Clone or fork this repo and `cd` to it
1. And then `cargo run`

## Roadmap

The goal is to eventually implement almost every Spotify feature.

### High level requirements yet to be implemented

- Add songs to a playlist
- Be able to scroll through result pages in every view
- View Library "Made for you"

This table shows all that is possible with the Spotify API, what is implemented already, and whether that is essential.

| API method                                        | Implemented yet? | Explanation                                                                                                                                                  | Essential? |
| ------------------------------------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------- |
| track                                             | No               | returns a single track given the track's ID, URI or URL                                                                                                      | No         |
| tracks                                            | No               | returns a list of tracks given a list of track IDs, URIs, or URLs                                                                                            | No         |
| artist                                            | No               | returns a single artist given the artist's ID, URI or URL                                                                                                    | Yes        |
| artists                                           | No               | returns a list of artists given the artist IDs, URIs, or URLs                                                                                                | No         | Get Spotify catalog information about an artist's albums | Yes |
| artist_albums                                     | Yes              | Get Spotify catalog information about an artist's top 10 tracks by country.                                                                                  | Yes        |
| artist_top_tracks                                 | No               | Get Spotify catalog information about artists similar to an                                                                                                  | Yes        |
| artist_related_artists                            | No               | Get Spotify catalog information about artists similar to an identified artist. Similarity is based on analysis of the Spotify community's listening history. | Yes        |
| album                                             | Yes              | returns a single album given the album's ID, URIs or URL                                                                                                     | Yes        |
| albums                                            | No               | returns a list of albums given the album IDs, URIs, or URLs                                                                                                  | No         |
| search_album                                      | Yes              | Search album based on query                                                                                                                                  | Yes        |
| search_artist                                     | Yes              | Search artist based on query                                                                                                                                 | Yes        |
| search_track                                      | Yes              | Search track based on query                                                                                                                                  | Yes        |
| search_playlist                                   | Yes              | Search playlist based on query                                                                                                                               | Yes        |
| album_track                                       | Yes              | Get Spotify catalog information about an album's tracks                                                                                                      | Yes        |
| user                                              | No               | Gets basic profile information about a Spotify User                                                                                                          | No         |
| playlist                                          | Yes              | playlist                                                                                                                                                     | Yes        |
| current_user_playlists                            | Yes              | Get current user playlists without required getting his profile                                                                                              | Yes        |
| user_playlists                                    | No               | Gets playlists of a user                                                                                                                                     | No         |
| user_playlist                                     | No               | Gets playlist of a user                                                                                                                                      | No         |
| user_playlist_tracks                              | Yes              | Get full details of the tracks of a playlist owned by a user Yes                                                                                             |
| user_playlist_create                              | No               | Creates a playlist for a user                                                                                                                                | Yes        |
| user_playlist_change_detail                       | No               | Changes a playlist's name and/or public/private state                                                                                                        | Yes        |
| user_playlist_unfollow                            | No               | Unfollows (deletes) a playlist for a user                                                                                                                    | Yes        |
| user_playlist_add_track                           | No               | Adds tracks to a playlist                                                                                                                                    | Yes        |
| user_playlist_replace_track                       | No               | Replace all tracks in a playlist                                                                                                                             | No         |
| user_playlist_recorder_tracks                     | No               | Reorder tracks in a playlist                                                                                                                                 | No         |
| user_playlist_remove_all_occurrences_of_track     | No               | Removes all occurrences of the given tracks from the given playlist                                                                                          | No         |
| user_playlist_remove_specific_occurrenes_of_track | No               | Removes all occurrences of the given tracks from the given playlist                                                                                          | No         |
| user_playlist_follow_playlist                     | No               | Add the current authenticated user as a follower of a playlist.                                                                                              | Yes        |
| user_playlist_check_follow                        | No               | Check to see if the given users are following the given playlist                                                                                             | Yes        |
| me                                                | No               | Get detailed profile information about the current user.                                                                                                     | Yes        |
| current_user                                      | No               | Alias for `me`                                                                                                                                               | Yes        |
| current_user_playing_track                        | Yes              | Get information about the current users currently playing track.                                                                                             | Yes        |
| current_user_saved_albums                         | No               | Gets a list of the albums saved in the current authorized user's "Your Music" library                                                                        | Yes        |
| current_user_saved_tracks                         | Yes              | Gets the user's saved tracks or "Liked Songs"                                                                                                                |
| current_user_followed_artists                     | No               | Gets a list of the artists followed by the current authorized user                                                                                           | Yes        |
| current_user_saved_tracks_delete                  | No               | Remove one or more tracks from the current user's "Your Music" library.                                                                                      | Yes        |
| current_user_saved_tracks_contain                 | No               | Check if one or more tracks is already saved in the current Spotify user’s “Your Music” library.                                                             | Yes        |
| current_user_saved_tracks_add                     | Yes              | Save one or more tracks to the current user's "Your Music" library.                                                                                          | Yes        |
| current_user_top_artists                          | No               | Get the current user's top artists                                                                                                                           | Yes        |
| current_user_top_tracks                           | No               | Get the current user's top tracks                                                                                                                            | Yes        |
| current_user_recently_played                      | No               | Get the current user's recently played tracks                                                                                                                | Yes        |
| current_user_saved_albums_add                     | No               | Add one or more albums to the current user's "Your Music" library.                                                                                           | Yes        |
| current_user_saved_albums_delete                  | No               | Remove one or more albums from the current user's "Your Music" library.                                                                                      | Yes        |
| user_follow_artists                               | No               | Follow one or more artists                                                                                                                                   | Yes        |
| user_unfollow_artists                             | No               | Unfollow one or more artists                                                                                                                                 | Yes        |
| user_follow_users                                 | No               | Follow one or more users                                                                                                                                     | No         |
| user_unfollow_users                               | No               | Unfollow one or more users                                                                                                                                   | No         |
| featured_playlists                                | No               | Get a list of Spotify featured playlists                                                                                                                     | Yes        |
| new_releases                                      | No               | Get a list of new album releases featured in Spotify                                                                                                         | Yes        |
| categories                                        | No               | Get a list of new album releases featured in Spotify                                                                                                         | Yes        |
| recommendations                                   | No               | Get Recommendations Based on Seeds                                                                                                                           | Yes        |
| audio_features                                    | No               | Get audio features for a track                                                                                                                               | No         |
| audios_feature                                    | No               | Get Audio Features for Several Tracks                                                                                                                        | No         |
| audio_analysis                                    | No               | Get Audio Analysis for a Track                                                                                                                               | No         |
| device                                            | Yes              | Get a User’s Available Devices                                                                                                                               | Yes        |
| current_playback                                  | Yes              | Get Information About The User’s Current Playback                                                                                                            | Yes        |
| current_playing                                   | No               | Get the User’s Currently Playing Track                                                                                                                       | No         |
| transfer_playback                                 | No               | Transfer a User’s Playback                                                                                                                                   | No         |
| start_playback                                    | Yes              | Start/Resume a User’s Playback                                                                                                                               | Yes        |
| pause_playback                                    | Yes              | Pause a User’s Playback                                                                                                                                      | Yes        |
| next_track                                        | Yes              | Skip User’s Playback To Next Track                                                                                                                           | Yes        |
| previous_track                                    | Yes              | Skip User’s Playback To Previous Track                                                                                                                       | Yes        |
| seek_track                                        | No               | Seek To Position In Currently Playing Track                                                                                                                  | Yes        |
| repeat                                            | Yes              | Set Repeat Mode On User’s Playback                                                                                                                           | Yes        |
| volume                                            | No               | Set Volume For User’s Playback                                                                                                                               | No         |
| shuffle                                           | Yes              | Toggle Shuffle For User’s Playback                                                                                                                           | Yes        |

[spotify-dev]: https://developer.spotify.com/dashboard/applications
