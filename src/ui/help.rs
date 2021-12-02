use crate::user_config::KeyBindings;

pub fn get_help_docs(key_bindings: &KeyBindings) -> Vec<Vec<String>> {
  vec![
    vec![
      String::from("Scroll down to next result page"),
      key_bindings.next_page.to_string(),
      String::from("Pagination"),
    ],
    vec![
      String::from("Scroll up to previous result page"),
      key_bindings.previous_page.to_string(),
      String::from("Pagination"),
    ],
    vec![
      String::from("Jump to start of playlist"),
      key_bindings.jump_to_start.to_string(),
      String::from("Pagination"),
    ],
    vec![
      String::from("Jump to end of playlist"),
      key_bindings.jump_to_end.to_string(),
      String::from("Pagination"),
    ],
    vec![
      String::from("Jump to currently playing album"),
      key_bindings.jump_to_album.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Jump to currently playing artist's album list"),
      key_bindings.jump_to_artist_album.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Jump to current play context"),
      key_bindings.jump_to_context.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Increase volume by 10%"),
      key_bindings.increase_volume.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Decrease volume by 10%"),
      key_bindings.decrease_volume.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Skip to next track"),
      key_bindings.next_track.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Skip to previous track"),
      key_bindings.previous_track.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Seek backwards 5 seconds"),
      key_bindings.seek_backwards.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Seek forwards 5 seconds"),
      key_bindings.seek_forwards.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Toggle shuffle"),
      key_bindings.shuffle.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Copy url to currently playing song/episode"),
      key_bindings.copy_song_url.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Copy url to currently playing album/show"),
      key_bindings.copy_album_url.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Cycle repeat mode"),
      key_bindings.repeat.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Move selection left"),
      String::from("h | <Left Arrow Key> | <Ctrl+b>"),
      String::from("General"),
    ],
    vec![
      String::from("Move selection down"),
      String::from("j | <Down Arrow Key> | <Ctrl+n>"),
      String::from("General"),
    ],
    vec![
      String::from("Move selection up"),
      String::from("k | <Up Arrow Key> | <Ctrl+p>"),
      String::from("General"),
    ],
    vec![
      String::from("Move selection right"),
      String::from("l | <Right Arrow Key> | <Ctrl+f>"),
      String::from("General"),
    ],
    vec![
      String::from("Move selection to top of list"),
      String::from("H"),
      String::from("General"),
    ],
    vec![
      String::from("Move selection to middle of list"),
      String::from("M"),
      String::from("General"),
    ],
    vec![
      String::from("Move selection to bottom of list"),
      String::from("L"),
      String::from("General"),
    ],
    vec![
      String::from("Enter input for search"),
      key_bindings.search.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Pause/Resume playback"),
      key_bindings.toggle_playback.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Enter active mode"),
      String::from("<Enter>"),
      String::from("General"),
    ],
    vec![
      String::from("Go to audio analysis screen"),
      key_bindings.audio_analysis.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Go to playbar only screen (basic view)"),
      key_bindings.basic_view.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Go back or exit when nowhere left to back to"),
      key_bindings.back.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Select device to play music on"),
      key_bindings.manage_devices.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Show lyrics for the current song"),
      key_bindings.show_lyrics.to_string(),
      String::from("General"),
    ],
    vec![
      String::from("Enter hover mode"),
      String::from("<Esc>"),
      String::from("Selected block"),
    ],
    vec![
      String::from("Save track in list or table"),
      String::from("s"),
      String::from("Selected block"),
    ],
    vec![
      String::from("Start playback or enter album/artist/playlist"),
      key_bindings.submit.to_string(),
      String::from("Selected block"),
    ],
    vec![
      String::from("Play recommendations for song/artist"),
      String::from("r"),
      String::from("Selected block"),
    ],
    vec![
      String::from("Play all tracks for artist"),
      String::from("e"),
      String::from("Library -> Artists"),
    ],
    vec![
      String::from("Search with input text"),
      String::from("<Enter>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Move cursor one space left"),
      String::from("<Left Arrow Key>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Move cursor one space right"),
      String::from("<Right Arrow Key>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Delete entire input"),
      String::from("<Ctrl+l>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Delete text from cursor to start of input"),
      String::from("<Ctrl+u>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Delete text from cursor to end of input"),
      String::from("<Ctrl+k>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Delete previous word"),
      String::from("<Ctrl+w>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Jump to start of input"),
      String::from("<Ctrl+a>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Jump to end of input"),
      String::from("<Ctrl+e>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Escape from the input back to hovered block"),
      String::from("<Esc>"),
      String::from("Search input"),
    ],
    vec![
      String::from("Delete saved album"),
      String::from("D"),
      String::from("Library -> Albums"),
    ],
    vec![
      String::from("Delete saved playlist"),
      String::from("D"),
      String::from("Playlist"),
    ],
    vec![
      String::from("Follow an artist/playlist"),
      String::from("w"),
      String::from("Search result"),
    ],
    vec![
      String::from("Save (like) album to library"),
      String::from("w"),
      String::from("Search result"),
    ],
    vec![
      String::from("Play random song in playlist"),
      String::from("S"),
      String::from("Selected Playlist"),
    ],
    vec![
      String::from("Toggle sort order of podcast episodes"),
      String::from("S"),
      String::from("Selected Show"),
    ],
    vec![
      String::from("Add track to queue"),
      key_bindings.add_item_to_queue.to_string(),
      String::from("Hovered over track"),
    ],
  ]
}
