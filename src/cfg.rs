use std::fs;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::api::TaskRequest;

#[derive(Clone, Debug)]
pub struct ConfigManager {
    pub config: Option<Config>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub api_key: String,
    pub saved_tasks: Vec<SavedTask>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedTask {
    pub task: TaskRequest,
    pub name: String,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl ConfigManager {

    pub fn save_task(&mut self, task: SavedTask) {
        let config = self.config.as_mut().unwrap();
        config.saved_tasks.push(task);
    }

    pub fn get_saved_tasks(&self) -> Vec<SavedTask> {
        let config = self.config.as_ref().unwrap();
        config.saved_tasks.clone()
    }

    pub fn validate(&self) -> bool {
        let exists = fs::try_exists("Config.toml");
        exists.is_ok() && exists.unwrap()
    }
    pub fn load(&mut self) {
        let exists = self.validate();
        if !exists {
            self.config = Some(Config {
                api_key: String::new(),
                saved_tasks: Vec::new(),
            });
            return;
        }
        let contents = fs::read_to_string("Config.toml");
        if contents.is_err() {
            panic!("Failed to read config file");
        }
        let config: Result<Config, toml::de::Error> = toml::from_str(contents.unwrap().as_str());
        if config.is_err() {
            panic!("Failed to parse config file");
        }
        self.config = Some(config.unwrap());
    }
    pub fn save(&self) {
        let config = toml::to_string(&self.config);
        if config.is_err() {
            panic!("Failed to parse config file");
        }
        let rs = fs::write("Config.toml", config.unwrap());
        if rs.is_err() {
            panic!("Failed to write config file");
        }
    }
}
