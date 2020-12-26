use crate::network::{IoEvent, Network};
use crate::user_config::UserConfig;

use super::{
  util::{Flag, JumpDirection, Type},
  CliApp,
};

use anyhow::{anyhow, Result};
use clap::ArgMatches;

// Handle the different subcommands
pub async fn handle_matches(
  matches: &ArgMatches<'_>,
  cmd: String,
  net: Network<'_>,
  config: UserConfig,
) -> Result<String> {
  let mut cli = CliApp::new(net, config);

  cli.net.handle_network_event(IoEvent::GetDevices).await;
  cli
    .net
    .handle_network_event(IoEvent::GetCurrentPlayback)
    .await;

  let devices_list = match &cli.net.app.lock().await.devices {
    Some(p) => p
      .devices
      .iter()
      .map(|d| d.id.clone())
      .collect::<Vec<String>>(),
    None => Vec::new(),
  };

  // If the device_id is not specified, select the first available device
  let device_id = cli.net.client_config.device_id.clone();
  if device_id.is_none() || !devices_list.contains(&device_id.unwrap()) {
    // Select the first device available
    if let Some(d) = devices_list.get(0) {
      cli.net.client_config.set_device_id(d.clone())?;
    }
  }

  if let Some(d) = matches.value_of("device") {
    cli.set_device(d.to_string()).await?;
  }

  // Evalute the subcommand
  let output = match cmd.as_str() {
    "playback" => {
      let format = matches.value_of("format").unwrap();

      // Run the action, and print out the status
      // No 'else if's because multiple different commands are possible
      if matches.is_present("toggle") {
        cli.toggle_playback().await;
      }
      if let Some(d) = matches.value_of("transfer") {
        cli.transfer_playback(d).await?;
      }
      if matches.is_present("flags") {
        let flag = Flag::from_matches(matches);
        cli.mark(flag).await?;
      }
      if matches.is_present("jumps") {
        let direction = JumpDirection::from_matches(matches);
        cli.jump(direction).await;
      }
      if let Some(vol) = matches.value_of("volume") {
        cli.volume(vol.to_string()).await?;
      }

      // Print out the status if no errors were found
      cli.get_status(format.to_string()).await
    }
    "play" => {
      let format = matches.value_of("format").unwrap();
      if let Some(uri) = matches.value_of("uri") {
        cli.play_uri(uri.to_string()).await;
      } else if let Some(name) = matches.value_of("name") {
        let category = Type::play_from_matches(matches);
        cli
          .play(name.to_string(), category, matches.is_present("queue"))
          .await?;
      }

      // Could be made configurable in the future
      cli.get_status(format.to_string()).await
    }
    "query" => {
      let format = matches.value_of("format").unwrap().to_string();

      // Update the limits for the list and search functions
      // I think the small and big search limits are very confusing
      // so I just set them both to max, is this okay?
      if let Some(max) = matches.value_of("limit") {
        cli.update_query_limits(max.to_string()).await?;
      }

      if matches.is_present("list") {
        let category = Type::list_from_matches(matches);
        Ok(cli.list(category, &format).await)
      } else if let Some(search) = matches.value_of("search") {
        let category = Type::search_from_matches(matches);
        Ok(cli.query(search.to_string(), format, category).await)
      // Never called, just here for the compiler
      // Clap enforces that one of the things above is specified
      } else {
        Ok(String::new())
      }
    }
    // Never called, just here for the compiler
    // Clap enforces that one of the things above is specified
    _ => Ok(String::new()),
  };

  // Check if there was an error
  let api_error = cli.net.app.lock().await.api_error.clone();
  if api_error.is_empty() {
    output
  } else {
    Err(anyhow!("{}", api_error))
  }
}
