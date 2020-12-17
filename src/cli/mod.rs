mod cli_app;
mod cmd;

pub use cli_app::handle_matches;
pub use cmd::{play_subcommand, playback_subcommand, query_subcommand};
