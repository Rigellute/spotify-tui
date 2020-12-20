mod cli_app;
mod clap;

pub use cli_app::handle_matches;
pub use self::clap::{play_subcommand, playback_subcommand, query_subcommand};
