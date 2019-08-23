# Spotify TUI

A Spotify client for the terminal.

### TODO

- [ ] Add `clap` for the cli
- [x] Add search for track
- [x] Display search results in a table
- [x] Allow user to start playing that song
- [x] Show currently playing song
  - [ ] Can I connect to a socket to always be in sync? Or do I need to poll the API?
- [ ] Handle device offline. Can we have an offline mode?
- [ ] Create a `localhost` server with pretty landing page for spotify auth redirect. This should simply explain to the user that they need to copy the url into the terminal.
- [ ] Let the user choose which device they want to play songs on.
- [ ] Cache the currently selected `device_id`.
- [ ] Show a help dialog (or table?) with all the supported commands.
  - Could use a global callback of `?`
- [ ] Could show the song progress? Might be tricky.
- [ ] Add global callback to handle pressing `space`, which should pause/start the current song.
- [ ] Add search for album
- [ ] Add search for artist
- [x] Show users playlists on start up
- [ ] Within the song table view, let the user use `ctrl + d/u` to move through pages in search results
- [ ] Pass `playlist_id` into uri list when playing a song within a playlist
- [ ] Implement vim style `H M L` for jumping `high medium low` within the table view
- [ ] On pressing play, update the currently playing view

### Libraries used

- [tui-rs](https://github.com/fdehau/tui-rs)
- [rspotify](https://github.com/ramsayleung/rspotify)
