mod clap;
mod cli_app;

pub use self::clap::{play_subcommand, playback_subcommand, query_subcommand};
pub use cli_app::handle_matches;
