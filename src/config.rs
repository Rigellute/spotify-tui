use dirs;
use failure::err_msg;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub device_id: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Default::default()
    }

    fn get_config() -> Result<Config, failure::Error> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let file_name = "client.yml";
                let config_path = path.join(".config/spotify-tui");

                if config_path.exists() {
                    let config_string = fs::read_to_string(config_path.join(file_name))?;
                    let config_yml: Config = serde_yaml::from_str(&config_string)?;

                    Ok(config_yml)
                } else {
                    Err(err_msg("Config does not exist"))
                }
            }
            None => Err(err_msg("No $HOME directory found for client config")),
        }
    }

    pub fn get_config() -> Result<Config, failure::Error> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let file_name = "client.yml";
                let config_path = path.join(".config/spotify-tui");

                if config_path.exists() {
                    let config_string = fs::read_to_string(config_path.join(file_name))?;
                    let config_yml: Config = serde_yaml::from_str(&config_string)?;

                    Ok(config_yml)
                } else {
                    println!("Config does not exist, creating it");
                    fs::create_dir(&config_path)?;
                    let mut new_config = fs::File::create(&config_path.join(file_name))?;
                    let content = Config {
                        client_id: "abddfslkjsj1234".to_string(),
                        client_secret: "abddfslkjsj1234".to_string(),
                        device_id: None,
                    };

                    let content_yml = serde_yaml::to_string(&content)?;
                    write!(new_config, "{}", content_yml)?;
                    Err(err_msg(format!(
                        "Add your spotify client_id and client_secret to {}",
                        config_path.display()
                    )))
                }
            }
            None => Err(err_msg("No $HOME directory found for client config")),
        }
    }

    pub fn set_cached_device_token(&self, device_token: String) -> Result<(), failure::Error> {
        let mut output = fs::File::create(&self.path_to_cached_device_id)?;
        write!(output, "{}", device_token)?;

        Ok(())
    }
}
