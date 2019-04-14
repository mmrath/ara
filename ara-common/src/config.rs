use config::{Config, File};
use failure::{Error, ResultExt};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
use log::info;

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::INIT;

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Smtp {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Security {
    pub password_expiry_days: u32,
    pub user_activation_token_expiry_mins: u32,
    pub user_password_reset_token_expiry_mins: u32,
    #[serde(skip_serializing)]
    pub secret_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub template_dir: String,
    pub server: Server,
    pub security: Security,
    pub database: Database,
    pub smtp: Smtp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub port: u16,
    pub base_url: String,
}

impl AppConfig {
    //This must be called once from main
    pub fn init(config_root: &str, env: &str) {
        info!("Trying to load config from {} for env {}", config_root, env);
        let config = AppConfig::load_config(config_root, env).expect("Unable to load config");
        match APP_CONFIG.set(config) {
            Ok(_) => {
                info!(
                    "Configuration loaded successfully\n{}",
                    json!(Self::get()).to_string()
                );
            }
            Err(_) => {
                panic!("Failed to initialize configuration");
            }
        };
    }

    pub fn get() -> &'static AppConfig {
        APP_CONFIG.get().unwrap()
    }

    fn load_config(config_root: &str, env: &str) -> Result<Self, Error> {
        let mut config = Config::new();
        let cfg_dir = Path::new(config_root);
        let default_cfg = cfg_dir.join("default");

        let _ = config
            .merge(File::from(default_cfg))
            .context("Unable to load default config")?;
        let _ = config
            .merge(File::from(cfg_dir.join(env)).required(false))
            .context("Unable to load env specific file")?;

        let cfg = config.try_into().context("")?;
        Ok(cfg)
    }
}
