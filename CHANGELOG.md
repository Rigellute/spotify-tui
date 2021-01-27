# Changelog

## [Unreleased]

- Add ability to seek from the CLI [#692](https://github.com/Rigellute/spotify-tui/pull/692)
- Replace `clipboard` with `arboard` [#691](https://github.com/Rigellute/spotify-tui/pull/691)
- Handle invalid Client ID/Secret [#668](https://github.com/Rigellute/spotify-tui/pull/668)
- Implement some episode table functions [#698](https://github.com/Rigellute/spotify-tui/pull/698)
- Fix default liked, shuffle, etc. icons to more recognizable symbols [#702](https://github.com/Rigellute/spotify-tui/pull/702)
- Change `--like` that toggled the liked-state to explicit `--like` and `--dislike` flags [#717](https://github.com/Rigellute/spotify-tui/pull/717)

## [0.23.0] - 2021-01-06

### Fixed

- Fix app crash when pressing Enter before a screen has loaded [#599](https://github.com/Rigellute/spotify-tui/pull/599)
- Make layout more responsive to large/small screens [#502](https://github.com/Rigellute/spotify-tui/pull/502)
- Fix use of incorrect playlist index when playing from an associated track table [#632](https://github.com/Rigellute/spotify-tui/pull/632)
- Fix flickering help menu in small screens [#638](https://github.com/Rigellute/spotify-tui/pull/638)
- Optimize seek [#640](https://github.com/Rigellute/spotify-tui/pull/640)
- Fix centering of basic_view [#664](https://github.com/Rigellute/spotify-tui/pull/664)

### Added

- Implement next/previous page behavior for the Artists table [#604](https://github.com/Rigellute/spotify-tui/pull/604)
- Show saved albums when getting an artist [#612](https://github.com/Rigellute/spotify-tui/pull/612)
- Transfer playback when changing device [#408](https://github.com/Rigellute/spotify-tui/pull/408)
- Search using Spotify share URLs and URIs like the desktop client [#623](https://github.com/Rigellute/spotify-tui/pull/623)
- Make the liked icon configurable [#659](https://github.com/Rigellute/spotify-tui/pull/659)
- Add CLI for controlling Spotify [#645](https://github.com/Rigellute/spotify-tui/pull/645)
- Implement Podcasts Library page [#650](https://github.com/Rigellute/spotify-tui/pull/650)

## [0.22.0] - 2020-10-05

### Fixed

- Show ♥ next to album name in saved list [#540](https://github.com/Rigellute/spotify-tui/pull/540)
- Fix to be able to follow an artist in search result view [#565](https://github.com/Rigellute/spotify-tui/pull/565)
- Don't add analysis view to stack if already in it [#580](https://github.com/Rigellute/spotify-tui/pull/580)

### Added

- Add additional line of help to show that 'w' can be used to save/like an album [#548](https://github.com/Rigellute/spotify-tui/pull/548)
- Add handling Home and End buttons in user input [#550](https://github.com/Rigellute/spotify-tui/pull/550)
- Add `playbar_progress_text` to user config and upgrade tui lib [#564](https://github.com/Rigellute/spotify-tui/pull/564)
- Add basic playbar support for podcasts [#563](https://github.com/Rigellute/spotify-tui/pull/563)
- Add 'enable_text_emphasis' behavior config option [#573](https://github.com/Rigellute/spotify-tui/pull/573)
- Add next/prev page, jump to start/end to user config [#566](https://github.com/Rigellute/spotify-tui/pull/566)
- Add possibility to queue a song [#567](https://github.com/Rigellute/spotify-tui/pull/567)
- Add user-configurable header styling [#583](https://github.com/Rigellute/spotify-tui/pull/583)
- Show active keybindings in Help [#585](https://github.com/Rigellute/spotify-tui/pull/585)
- Full Podcast support [#581](https://github.com/Rigellute/spotify-tui/pull/581)

## [0.21.0] - 2020-07-24

### Fixed

- Fix typo in help menu [#485](https://github.com/Rigellute/spotify-tui/pull/485)

### Added

- Add save album on album view [#506](https://github.com/Rigellute/spotify-tui/pull/506)
- Add feature to like a song from basic view [#507](https://github.com/Rigellute/spotify-tui/pull/507)
- Enable Unix and Linux shortcut keys in the input [#511](https://github.com/Rigellute/spotify-tui/pull/511)
- Add album artist field to full album view [#519](https://github.com/Rigellute/spotify-tui/pull/519)
- Handle track saving in non-album contexts (eg. playlist/Made for you). [#525](https://github.com/Rigellute/spotify-tui/pull/525)

## [0.20.0] - 2020-05-28

### Fixed

- Move pagination instructions to top of help menu [#442](https://github.com/Rigellute/spotify-tui/pull/442)

### Added

- Add user configuration toggle for the loading indicator [#447](https://github.com/Rigellute/spotify-tui/pull/447)
- Add support for saving an album and following an artist in artist view [#445](https://github.com/Rigellute/spotify-tui/pull/445)
- Use the `▶` glyph to indicate the currently playing song [#472](https://github.com/Rigellute/spotify-tui/pull/472)
- Jump to play context (if available) - default binding is `o` [#474](https://github.com/Rigellute/spotify-tui/pull/474)

## [0.19.0] - 2020-05-04

### Fixed

- Fix re-authentication [#415](https://github.com/Rigellute/spotify-tui/pull/415)
- Fix audio analysis feature [#435](https://github.com/Rigellute/spotify-tui/pull/435)

### Added

- Add more readline shortcuts to the search input [#425](https://github.com/Rigellute/spotify-tui/pull/425)

## [0.18.0] - 2020-04-21

### Fixed

- Fix crash when opening playlist [#398](https://github.com/Rigellute/spotify-tui/pull/398)
- Fix crash when there are no artists avaliable [#388](https://github.com/Rigellute/spotify-tui/pull/388)
- Correctly handle playlist unfollowing [#399](https://github.com/Rigellute/spotify-tui/pull/399)

### Added

- Allow specifying alternative config file path [#391](https://github.com/Rigellute/spotify-tui/pull/391)
- List artists names in the album view [#393](https://github.com/Rigellute/spotify-tui/pull/393)
- Add confirmation modal for delete playlist action [#402](https://github.com/Rigellute/spotify-tui/pull/402)

## [0.17.1] - 2020-03-30

### Fixed

- Artist name in songs block [#365](https://github.com/Rigellute/spotify-tui/pull/365) (fixes regression)
- Add basic view key binding to help menu

## [0.17.0] - 2020-03-20

### Added

- Show if search results are liked/followed [#342](https://github.com/Rigellute/spotify-tui/pull/342)
- Show currently playing track in song search menu and play through the searched tracks [#343](https://github.com/Rigellute/spotify-tui/pull/343)
- Add a "basic view" that only shows the playbar. Press `B` to get there [#344](https://github.com/Rigellute/spotify-tui/pull/344)
- Show currently playing top track [#347](https://github.com/Rigellute/spotify-tui/pull/347)
- Press shift-s (`S`) to pick a random song on track-lists [#339](https://github.com/Rigellute/spotify-tui/pull/339)

### Fixed

- Prevent search when there is no input [#351](https://github.com/Rigellute/spotify-tui/pull/351)

## [0.16.0] - 2020-03-12

### Fixed

- Fix empty UI when pressing escape in the device and analysis views [#315](https://github.com/Rigellute/spotify-tui/pull/315)
- Fix slow and frozen UI by implementing an asynchronous runtime for network events [#322](https://github.com/Rigellute/spotify-tui/pull/322). This fixes issues #24, #92, #207 and #218. Read more [here](https://keliris.dev/improving-spotify-tui/).

## [0.15.0] - 2020-02-24

- Add experimental audio visualizer (press `v` to navigate to it). The feature uses the audio analysis data from Spotify and animates the pitch information.
- Display Artist layout when searching an artist url [#298](https://github.com/Rigellute/spotify-tui/pull/298)
- Add pagination to the help menu [#302](https://github.com/Rigellute/spotify-tui/pull/302)

## [0.14.0] - 2020-02-11

### Added

- Add high-middle-low navigation (`H`, `M`, `L` respectively) for jumping around lists [#234](https://github.com/Rigellute/spotify-tui/pull/234).
- Play every known song with `e` [#228](https://github.com/Rigellute/spotify-tui/pull/228)
- Search album by url: paste a spotify album link into the search input to go to that album [#281](https://github.com/Rigellute/spotify-tui/pull/281)
- Implement 'Made For You' section of Library [#278](https://github.com/Rigellute/spotify-tui/pull/278)
- Add user theme configuration [#284](https://github.com/Rigellute/spotify-tui/pull/284)
- Allow user to define the volume increment [#288](https://github.com/Rigellute/spotify-tui/pull/288)

### Fixed

- Fix crash on small terminals [#231](https://github.com/Rigellute/spotify-tui/pull/231)

## [0.13.0] - 2020-01-26

### Fixed

- Don't error if failed to open clipboard [#217](https://github.com/Rigellute/spotify-tui/pull/217)
- Fix scrolling beyond the end of pagination. [#216](https://github.com/Rigellute/spotify-tui/pull/216)
- Add copy album url functionality [#226](https://github.com/Rigellute/spotify-tui/pull/226)

### Added

- Allow user to configure the port for the Spotify auth Redirect URI [#204](https://github.com/Rigellute/spotify-tui/pull/204)
- Add play recommendations for song/artist on pressing 'r' [#200](https://github.com/Rigellute/spotify-tui/pull/200)
- Added continuous deployment for Windows [#222](https://github.com/Rigellute/spotify-tui/pull/222)

### Changed

- Change behavior of previous button (`p`) to mimic behavior in official Spotify client. When the track is more than three seconds in, pressing previous will restart the track. When less than three seconds it will jump to previous. [#219](https://github.com/Rigellute/spotify-tui/pull/219)

## [0.12.0] - 2020-01-23

### Added

- Add Windows support [#99](https://github.com/Rigellute/spotify-tui/pull/99)
- Add support for Related artists and top tacks [#191](https://github.com/Rigellute/spotify-tui/pull/191)

## [0.11.0] - 2019-12-23

### Added

- Add support for adding an album and following a playlist. NOTE: that this will require the user to grant more permissions [#172](https://github.com/Rigellute/spotify-tui/pull/172)
- Add shortcuts to jump to the start or the end of a playlist [#167](https://github.com/Rigellute/spotify-tui/pull/167)
- Make seeking amount configurable [#168](https://github.com/Rigellute/spotify-tui/pull/168)

### Fixed

- Fix playlist index after search [#177](https://github.com/Rigellute/spotify-tui/pull/177)
- Fix cursor offset in search input [#183](https://github.com/Rigellute/spotify-tui/pull/183)

### Changed

- Remove focus on input when jumping back [#184](https://github.com/Rigellute/spotify-tui/pull/184)
- Pad strings in status bar to prevent reformatting [#188](https://github.com/Rigellute/spotify-tui/pull/188)

## [0.10.0] - 2019-11-30

### Added

- Added pagination to user playlists [#150](https://github.com/Rigellute/spotify-tui/pull/150)
- Add ability to delete a saved album (hover over the album you wish to delete and press `D`) [#152](https://github.com/Rigellute/spotify-tui/pull/152)
- Add support for following/unfollowing artists [#155](https://github.com/Rigellute/spotify-tui/pull/155)
- Add hotkey to copy url of currently playing track (default binding is `c`)[#156](https://github.com/Rigellute/spotify-tui/pull/156)

### Fixed

- Refine Spotify result limits, which should fit your current terminal size. Most notably this will increase the number of results from a search [#154](https://github.com/Rigellute/spotify-tui/pull/154)
- Navigation from "Liked Songs" [#151](https://github.com/Rigellute/spotify-tui/pull/151)
- App hang upon trying to authenticate with Spotify on FreeBSD [#148](https://github.com/Rigellute/spotify-tui/pull/148)
- Showing "Release Date" in saved albums table [#162](https://github.com/Rigellute/spotify-tui/pull/162)
- Showing "Length" in library -> recently played table [#164](https://github.com/Rigellute/spotify-tui/pull/164)
- Typo: "AlbumTracks" -> "Albums" [#165](https://github.com/Rigellute/spotify-tui/pull/165)
- Janky volume control [#166](https://github.com/Rigellute/spotify-tui/pull/166)
- Volume bug that would prevent volumes of 0 and 100 [#170](https://github.com/Rigellute/spotify-tui/pull/170)
- Playing a wrong track in playlist [#173](https://github.com/Rigellute/spotify-tui/pull/173)

## [0.9.0] - 2019-11-13

### Added

- Add custom keybindings feature. Check the README for an example configuration [#112](https://github.com/Rigellute/spotify-tui/pull/112)

### Fixed

- Fix panic when seeking beyond track boundaries [#124](https://github.com/Rigellute/spotify-tui/pull/124)
- Add scrolling to library album list. Can now use `ctrl+d/u` to scroll between result pages [#128](https://github.com/Rigellute/spotify-tui/pull/128)
- Fix showing wrong album in library album view - [#130](https://github.com/Rigellute/spotify-tui/pull/130)
- Fix scrolling in table views [#135](https://github.com/Rigellute/spotify-tui/pull/135)
- Use space more efficiently in small terminals [#143](https://github.com/Rigellute/spotify-tui/pull/143)

## [0.8.0] - 2019-10-29

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
