# Spotify TUI

A Spotify client for the terminal.

### Limitations

It seems you need to have the spotify app open in order to play songs.

### TODO

- [ ] Add `clap` for the cli
- [ ] Add global callback to handle pressing `space`, which should pause/start the current song.
- [ ] Create a `localhost` server with pretty landing page for spotify auth redirect. This should simply explain to the user that they need to copy the url into the terminal.
- [x] Implement routes (e.g. search, open playlist, go back to search or search, open artist, open album, play track, go back to album etc.)
- [x] Implement stack navigation for routes
- [ ] Add recommendations and recently played to home page
- [ ] Implement vim style `H M L` for jumping `high medium low` within the table view?
- [ ] Let the user press `i` on selected song to get more information about it
- [ ] Let user "like" a song could use `*` event?
- [ ] Let user define custom theme?
- [ ] Within the list/table views, let the user use `ctrl + d/u` to move up and down pages in search results
- [ ] Implement track seek
- [ ] Implement album screen (could just be selectable table list?)
- [ ] Implement artist screen (show top tracks, albums, other artist recommendations?)
- [ ] Let user press `p` to start playing an album or playlist
- [x] Add error block on api errors
- [x] Add search for album
- [x] Add search for artist
- [x] Add search for track
- [x] Allow user to start playing that song
- [x] Cache the currently selected `device_id`.
- [x] Could show the song progress? Might be tricky. Use termion [gauge](https://github.com/fdehau/tui-rs/blob/master/examples/gauge.rs)?
- [x] Display search results in a table
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
