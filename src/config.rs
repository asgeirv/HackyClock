use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub alarm_time: AlarmTime,
}

#[derive(Deserialize, Debug)]
pub struct AlarmTime {
    hour: u8,
    minute: u8,
}

pub fn read() -> anyhow::Result<Config> {
    let file = File::open("config.yml")?;
    Ok(serde_yaml::from_reader(file)?)
}
