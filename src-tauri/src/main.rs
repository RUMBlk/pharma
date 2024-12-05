// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use toml;
use serde::Deserialize;
use sea_orm;

#[derive(Debug, Deserialize)]
struct Config {
    db_user: String,
    db_pass: String,
    db_host: String,
    db_port: String,
    db_name: String,
}

impl Config {
    pub fn db_uri(&self) -> String {
        format!("mysql://{}:{}@{}:{}/{}", self.db_user, self.db_pass, self.db_host, self.db_port, self.db_name)
    }
}

#[tokio::main]
async fn main() {
    let config_content = fs::read_to_string("Config.toml").expect("Помилка читання файла Config.toml");
    let config: Config = toml::from_str(&config_content).expect("Помилка десеріалізації файла Config.toml");
    println!("{config:?} {}", config.db_uri());
    let db = sea_orm::Database::connect(config.db_uri()).await.expect("Не вдалося під'єднатися до бази даних");
    pharma_lib::run(db)
}
