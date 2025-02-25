use deadpool_postgres::{Config as PgConfig};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Settings {
    pub ip: String,
    pub port: String,
    pub jwt_secret: String,
    pub path_to_image_static_dir: String,
    pub path_to_document_static_dir: String,
    pub pg: PgConfig,
    pub log: LogConfig,
    pub path_to_cert_key: String,
    pub path_to_cert_file: String,
    pub kimi_secret: String
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct LogConfig {
    pub level: String,
    pub color_mode: String, // always auto never
}


impl Settings {}
