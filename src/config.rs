use std::collections::HashMap;

use config::ConfigError;
use irc::client::prelude::Config;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("could not find setting {0}")]
    MissingSetting(&'static str),
    #[error(transparent)]
    ConfigReadError(#[from] ConfigError),
}

pub fn get_user_config(token: &str) -> Result<Config, SettingsError> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings").required(false))?
        .merge(config::Environment::with_prefix("TWITCH"))?;
    let config = settings.try_into::<HashMap<String, String>>()?;

    let name = config
        .get("name")
        .ok_or(SettingsError::MissingSetting("name"))?;

    let channel = config
        .get("channel")
        .map(|n| {
            if n.starts_with('#') {
                n.clone()
            } else {
                format!("#{}", n)
            }
        })
        .ok_or(SettingsError::MissingSetting("channel"))?;

    Ok(Config {
        nickname: Some(name.clone()),
        password: Some(token.to_owned()),
        server: Some("irc.chat.twitch.tv".to_owned()),
        port: Some(6697),
        channels: vec![channel],
        ..Default::default()
    })
}
