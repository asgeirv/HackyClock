use std::fs::File;

use chrono::Weekday;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Config {
    pub alarms: Vec<Alarm>,
    pub audio_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Alarm {
    pub hour: u8,
    pub minute: u8,
    #[serde(deserialize_with = "deserialize_weekdays")]
    pub weekdays: Vec<Weekday>,
}

fn deserialize_weekdays<'de, D>(deserializer: D) -> Result<Vec<Weekday>, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|s| {
            s.parse()
                .map_err(|_| D::Error::custom("unable to parse weekday"))
        })
        .collect()
}

pub fn read() -> anyhow::Result<Config> {
    let file = File::open("config.yml")?;
    Ok(serde_yaml::from_reader(file)?)
}
