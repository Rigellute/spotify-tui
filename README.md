# Spotify TUI

A Spotify client for the terminal.

### Limitations

It seems you need to have the spotify app open in order to play songs.

### TODO

- [ ] Add `clap` for the cli
- [ ] Add global callback to handle pressing `space`, which should pause/start the current song.
- [ ] Add search for album
- [ ] Add search for artist
- [ ] Cache the currently selected `device_id`.
- [ ] Create a `localhost` server with pretty landing page for spotify auth redirect. This should simply explain to the user that they need to copy the url into the terminal.
- [ ] Handle device offline. Can we have an offline mode?
- [ ] Implement routes (e.g. search, open playlist, go back to search or search, open artist, open album, play track, go back to album etc.)
- [ ] Implement stack navigation for routes
- [ ] Implement vim style `H M L` for jumping `high medium low` within the table view
- [ ] Let the user choose which device they want to play songs on.
- [ ] Let the user press `i` on selected song to get more information about it
- [ ] Let user "like" a song could use `*`?
- [ ] Let user define custom theme?
- [ ] On in active blocks show current selection as bold but as WHITE (nor CYAN)
- [ ] Pass `playlist_id` into uri list when playing a song within a playlist
- [ ] Show a help dialog (or table?) with all the supported commands. Could use a global callback of `?`
- [ ] Show search results for playlists, tracks and albums
- [ ] Within the song table view, let the user use `ctrl + d/u` to move through pages in search results
- [x] Add error block on api errors
- [x] Add search for track
- [x] Allow user to start playing that song
- [x] Could show the song progress? Might be tricky. Use termion [gauge](https://github.com/fdehau/tui-rs/blob/master/examples/gauge.rs)?
- [x] Display search results in a table
- [x] On pressing play, update the currently playing view
- [x] Show currently playing song: Can I connect to a socket to always be in sync? Or do I need to poll the API?
- [x] Show users playlists on start up

### Libraries used

- [tui-rs](https://github.com/fdehau/tui-rs)
- [rspotify](https://github.com/ramsayleung/rspotify)
