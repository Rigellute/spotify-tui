# Spotify TUI

A Spotify client for the terminal written in Rust.

![Demo](https://user-images.githubusercontent.com/12150276/64544624-3a798480-d31f-11e9-8ecf-35e369563a26.gif)

### Limitations

You need to have the spotify app open in order to play songs, but you can control all your devices with this app.

### Installation

TODO

### Connecting to Spotify’s API

`spotify-tui` needs to connect to Spotify’s API in order to find music by
name, play tracks etc.

To get this to work, you first need to sign up (or into) Spotify’s
developer site and [create an _Application_][spotify-dev]. Once you’ve
done so, you can find its `Client ID` and `Client Secret` values and
enter them into your `spotify-tui` file at `${HOME}/.config/spotify-tui/client.yml`.

```
client_id: abc01de2fghijk345lmnop
client_secret: qr6stu789vwxyz
```

When you start `spotify-tui` with this set, you will be redirected to an official spotify webpage to ask you for permissions.

Once accepted you will be redirected to `localhost` (this is temporary: eventually this will be a webpage with these instructions). Copy the URL and paste into the prompt back in the terminal. And now you are ready to use the `spotify-tui`!

[spotify-dev]: https://developer.spotify.com/my-applications/#!/applications/create

### TODO

- [ ] Add `clap` for the cli
- [ ] Add recommendations and recently played to home page
- [ ] Create a pretty landing page for spotify auth redirect. This should simply explain to the user that they need to copy the url into the terminal. Should deploy this to as a static site.
- [ ] Fix previous state on navigating back: Make the stack navigation contain active block state (including all the indexes)
- [ ] Implement a key event for opening the context of the currently playing track. Perhaps need to focus on player and from there jump to album/playlist
- [ ] Implement artist screen (show top tracks, albums, other artist recommendations?)
- [ ] Implement left right events to move cursor within the input
- [ ] Implement track seek
- [ ] Implement vim style `H M L` for jumping `high medium low` within the table view?
- [ ] Let the user press `i` on selected song to get more information about it
- [ ] Let user "like" a song could use `*` event?
- [ ] Let user define custom theme?
- [ ] Let user press `p` to start playing an album or playlist
- [ ] Within the list/table views, let the user use `ctrl + d/u` to move up and down pages in search results
- [x] Add error block on api errors
- [x] Add global callback to handle pressing `space`, which should pause/start the current song.
- [x] Add search for album
- [x] Add search for artist
- [x] Add search for track
- [x] Allow user to start playing that song
- [x] Cache the currently selected `device_id`.
- [x] Could show the song progress? Might be tricky. Use termion [gauge](https://github.com/fdehau/tui-rs/blob/master/examples/gauge.rs)?
- [x] Display search results in a table
- [x] Implement album screen (could just be selectable table list?)
- [x] Implement routes (e.g. search, open playlist, go back to search or search, open artist, open album, play track, go back to album etc.)
- [x] Implement shuffle
- [x] Implement stack navigation for routes
- [x] Let the user choose which device they want to play songs on.
- [x] On in active blocks show current selection as bold but as WHITE (nor CYAN)
- [x] On pressing play, update the currently playing view
- [x] Pass `playlist_id` into uri list when playing a song within a playlist
- [x] Show a help dialog (or table?) with all the supported commands. Could use a global callback of `?`
- [x] Show currently playing song: Can I connect to a socket to always be in sync? Or do I need to poll the API?
- [x] Show search results for playlists, tracks and albums
- [x] Show users playlists on start up

### Libraries used

- [tui-rs](https://github.com/fdehau/tui-rs)
- [rspotify](https://github.com/ramsayleung/rspotify)
