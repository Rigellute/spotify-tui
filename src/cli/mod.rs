mod cli;
mod cmd;

pub use cli::handle_matches;
pub use cmd::{play_subcommand, playback_subcommand, query_subcommand};
