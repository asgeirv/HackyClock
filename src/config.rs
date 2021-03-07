use std::fs::File;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use chrono::Weekday;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::de::Error;
use serde::{Deserialize, Deserializer};

const CONFIG_PATH: &str = "config.yml";

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

pub struct ConfigWatcher {
    // Field exists to keep the file watcher watching
    _watcher: RecommendedWatcher,
    rx: Receiver<notify::DebouncedEvent>,
}

impl ConfigWatcher {
    pub fn poll(&self) -> Option<anyhow::Result<Config>> {
        if let Ok(notify::DebouncedEvent::Write(_)) = self.rx.try_recv() {
            Some(read())
        } else {
            None
        }
    }
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
    let file = File::open(CONFIG_PATH)?;
    Ok(serde_yaml::from_reader(file)?)
}

pub fn watch() -> anyhow::Result<ConfigWatcher> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::watcher(tx, Duration::from_millis(500))?;
    watcher.watch(CONFIG_PATH, RecursiveMode::NonRecursive)?;

    Ok(ConfigWatcher { _watcher: watcher, rx })
}
