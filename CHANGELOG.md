# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Keep format of highlighted track when it is playing - [#44](https://github.com/Rigellute/spotify-tui/pull/44) thanks to [@jfaltis](https://github.com/jfaltis)
- Start a web server on localhost to display a simple webpage for the Redirect URI. Should hopefully improve the onboarding experience.
- Add ability to skip to tracks using `n` for next and `p` for previous - thanks to [@samcal](https://github.com/samcal)
- Implement seek functionality - you can now use `<` to seek backwards 5 seconds and `>` to go forwards 5 seconds

## [0.0.5] - 2019-10-11

### Added

- Add `Ctrl-r` to cycle repeat mode ([@baxtea](https://github.com/baxtea))
- Refresh token when token expires ([@fangyi-zhou](https://github.com/fangyi-zhou))
- Upgrade `rspotify` to fix [#39](https://github.com/Rigellute/spotify-tui/issues/39) ([@epwalsh](https://github.com/epwalsh))

### Changed

- Fix duplicate albums showing in artist discographies ([@baxtea](https://github.com/baxtea))
- Slightly better error message with some debug tips when tracks fail to play

## [0.0.4] - 2019-10-05

### Added

- Can now install `spotify-tui` using `brew reinstall Rigellute/tap/spotify-tui` and `cargo install spotify-tui` 🎉
- Credentials (auth token, chosen device, and CLIENT_ID & CLIENT_SECRET) are now all stored in the same place (`${HOME}/.config/spotify-tui/client.yml`), which closes [this issue](https://github.com/Rigellute/spotify-tui/issues/4)

## [0.0.3] - 2019-10-04

### Added

- Improved onboarding experience
- On first startup instructions will (hopefully) guide the user on how to get setup

## [0.0.2] - 2019-09-17

### Added

- General navigation improvements
- Improved search input: it should now behave how one would expect
- Add `Ctrl-d/u` for scrolling up and down through result pages (currently only implemented for "Liked Songs")
- Minor theme improvements
- Make tables responsive
- Implement resume playback feature
- Add saved albums table
- Show which track is currently playing within a table or list
- Add `a` event to jump to currently playing track's album
- Add `s` event to save a track from within the "Recently Played" view (eventually this should be everywhere)
- Add `Ctrl-s` to toggle shuffle
- Add the following journey: search -> select artist and see their albums -> select album -> go to album and play tracks
