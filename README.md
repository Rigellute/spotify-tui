# Spotify TUI

A Spotify client for the terminal.

### TODO

- [x] Add search for track
- [x] Display search results in a table
- [x] Show currently playing song
  - [ ] Can I connect to a socket to always be in sync? Or do I need to poll the API?
- [ ] Handle device offline. Can we have an offline mode?
- [ ] Create a `localhost` server with pretty landing page for spotify auth redirect. This should simply explain to the user that they need to copy the url into the terminal.
- [ ] Let the user choose which device they want to play songs on.
- [ ] Cache the currently selected `device_id`.
- [ ] Show a help dialog (or table?) with all the supported commands.
  - Could use a global callback of `?`
- [ ] Figure out how to position views properly - put the currently playing component at the bottom.
- [ ] Could show the song progress? Might be tricky.
- [ ] Add global callback to handle pressing `space`, which should pause/start the current song.
- [ ] Add search for album
- [ ] Add search for artist
- [ ] Show users playlists on start up
- [ ] Within the playlist view (one big table?), let the user use `ctrl + d/u` to move through pages in search results

### Libraries used

- [cursive](https://github.com/gyscos/cursive)
- [rspotify](https://github.com/ramsayleung/rspotify)
