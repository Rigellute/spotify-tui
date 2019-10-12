pub fn get_help_docs() -> Vec<Vec<&'static str>> {
    vec![
        vec!["General", "a", "Jump to currently playing album"],
        vec!["General", "s", "Save track"],
        vec!["General", "n", "Skip to next track"],
        vec!["General", "p", "Skip to previous track"],
        vec!["General", "<Ctrl+s>", "Toggle shuffle"],
        vec!["General", "<Ctrl+r>", "Cycle repeat mode"],
        vec!["General", "h | <Left Arrow Key>", "Move selection left"],
        vec![
            "General",
            "j | <Down Arrow Key> | <Ctrl+n>",
            "Move selection down",
        ],
        vec![
            "General",
            "k | <Up Arrow Key> | <Ctrl+p>",
            "Move selection up",
        ],
        vec!["General", "l | <Right Arrow Key>", "Move right"],
        vec!["General", "/", "Enter input for search"],
        vec!["General", "<Space>", "Pause/Resume playback"],
        vec!["General", "<Enter>", "Enter active mode"],
        vec![
            "General",
            "q | -",
            "Go back or exit when nowhere left to back to",
        ],
        vec!["General", "d", "Select device to play music on"],
        vec!["Selected block", "<Esc>", "Enter hover mode"],
        vec![
            "Selected block",
            "<Enter>",
            "Start playback or enter album/artist/playlist",
        ],
        vec!["Search input", "<Ctrl+u>", "Delete entire input"],
        vec!["Search input", "<Enter>", "Search with input text"],
        vec![
            "Search input",
            "<Left Arrow Key>",
            "Move cursor one space left",
        ],
        vec![
            "Search input",
            "<Right Arrow Key>",
            "Move cursor one space right",
        ],
        vec!["Search input", "<Ctrl+a>", "Jump to start of input"],
        vec!["Search input", "<Ctrl+e>", "Jump to end of input"],
        vec![
            "Search input",
            "<Esc>",
            "Escape from the input back to hovered block",
        ],
        vec!["Pagination", "<Ctrl+d>", "Scroll down to next result page"],
        vec![
            "Pagination",
            "<Ctrl+u>",
            "Scroll up to previous result page",
        ],
    ]
}
