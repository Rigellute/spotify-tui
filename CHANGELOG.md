# Changelog

## [Unreleased]

## [0.8.0] - 2019-10-29

- Add custom keybindings feature. Check the README for an example configuration [#112](https://github.com/Rigellute/spotify-tui/pull/112)
- Fix panic when seeking beyond track boundaries [#124](https://github.com/Rigellute/spotify-tui/pull/124)
- Add scrolling to library album list. Can now use `ctrl+d/u` to scroll between result pages [#128](https://github.com/Rigellute/spotify-tui/pull/128)

### Added

- Improve onboarding: auto fill the redirect url [#98](https://github.com/Rigellute/spotify-tui/pull/98)
- Indicate if a track is "liked" in Recently Played, Album tracks and song list views using "♥" - [#103](https://github.com/Rigellute/spotify-tui/pull/103)
- Add ability to toggle the saved state of a track: pressing `s` on an already saved track will unsave it. [#104](https://github.com/Rigellute/spotify-tui/pull/104)
- Add collaborative playlists scope. You'll need to reauthenticate due to this change. [#115](https://github.com/Rigellute/spotify-tui/pull/115)
- Add Ctrl-f and Ctrl-b emacs style keybindings for left and right motion. [#114](https://github.com/Rigellute/spotify-tui/pull/114)

### Fixed

- Fix app crash when pressing `enter`, `q` and then `down`. [#109](https://github.com/Rigellute/spotify-tui/pull/109)
- Fix trying to save a track in the album view [#119](https://github.com/Rigellute/spotify-tui/pull/119)
- Fix UI saved indicator when toggling saved track [#119](https://github.com/Rigellute/spotify-tui/pull/119)

## [0.7.0] - 2019-10-20

- Implement library "Artists" view - [#67](https://github.com/Rigellute/spotify-tui/pull/67) thanks [@svenvNL](https://github.com/svenvNL). NOTE that this adds an additional scope (`user-follow-read`), so you'll be prompted to grant this new permissions when you upgrade.
- Fix searching with non-english characters - [#30](https://github.com/Rigellute/spotify-tui/pull/30). Thanks to [@fangyi-zhou](https://github.com/fangyi-zhou)
- Remove hardcoded country (was always set to UK). We now fetch the user to get their country. [#68](https://github.com/Rigellute/spotify-tui/pull/68). Thanks to [@svenvNL](https://github.com/svenvNL)
- Save currently playing track - the playbar is now selectable/hoverable [#80](https://github.com/Rigellute/spotify-tui/pull/80)
- Lay foundation for showing if a track is saved. You can now see if the currently playing track is saved (indicated by ♥)

## [0.6.0] - 2019-10-14

### Added

- Start a web server on localhost to display a simple webpage for the Redirect URI. Should hopefully improve the onboarding experience.
- Add ability to skip to tracks using `n` for next and `p` for previous - thanks to [@samcal](https://github.com/samcal)
- Implement seek functionality - you can now use `<` to seek backwards 5 seconds and `>` to go forwards 5 seconds
- The event `A` will jump to the album list of the first artist in the track's artists list - closing [#45](https://github.com/Rigellute/spotify-tui/issues/45)
- Add volume controls - use `-` to decrease and `+` to increase volume in 10% increments. Closes [#57](https://github.com/Rigellute/spotify-tui/issues/57)

### Fixed

- Keep format of highlighted track when it is playing - [#44](https://github.com/Rigellute/spotify-tui/pull/44) thanks to [@jfaltis](https://github.com/jfaltis)
- Search input bug: Fix "out-of-bounds" crash when pressing left too many times [#63](https://github.com/Rigellute/spotify-tui/issues/63)
- Search input bug: Fix issue that backspace always deleted the end of input, not where the cursor was [#33](https://github.com/Rigellute/spotify-tui/issues/33)

## [0.5.0] - 2019-10-11

### Added

- Add `Ctrl-r` to cycle repeat mode ([@baxtea](https://github.com/baxtea))
- Refresh token when token expires ([@fangyi-zhou](https://github.com/fangyi-zhou))
- Upgrade `rspotify` to fix [#39](https://github.com/Rigellute/spotify-tui/issues/39) ([@epwalsh](https://github.com/epwalsh))

### Changed

- Fix duplicate albums showing in artist discographies ([@baxtea](https://github.com/baxtea))
- Slightly better error message with some debug tips when tracks fail to play

## [0.4.0] - 2019-10-05

### Added

- Can now install `spotify-tui` using `brew reinstall Rigellute/tap/spotify-tui` and `cargo install spotify-tui` 🎉
- Credentials (auth token, chosen device, and CLIENT_ID & CLIENT_SECRET) are now all stored in the same place (`${HOME}/.config/spotify-tui/client.yml`), which closes [this issue](https://github.com/Rigellute/spotify-tui/issues/4)

## [0.3.0] - 2019-10-04

### Added

- Improved onboarding experience
- On first startup instructions will (hopefully) guide the user on how to get setup

## [0.2.0] - 2019-09-17

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

# What is this?

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
