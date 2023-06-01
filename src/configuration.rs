use serde::{Serialize, Deserialize};
use std::io;
use envconfig::Envconfig;

const APP_NAME: &str = "gpterm";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GptermConfig {
    pub api_key: String,
}

#[derive(Envconfig)]
struct EnvironmentConfig {
    #[envconfig(from = "GPTERM_API_KEY", default = "")]
    pub api_key: String,
}

impl ::std::default::Default for GptermConfig {
    fn default() -> Self { Self { api_key: "".into() } }
}

pub fn get() -> Result<GptermConfig, confy::ConfyError> {
    let mut cfg: GptermConfig = confy::load(APP_NAME, None)?;
    let env_cfg = EnvironmentConfig::init_from_env().unwrap();

    if !env_cfg.api_key.is_empty() {
        cfg.api_key = env_cfg.api_key;
    }

    if cfg.api_key.is_empty() {
        print_no_api_key_info();
        let mut buffer = String::new();
        let read_result = io::stdin().read_line(&mut buffer);

        if read_result.is_ok() {
            cfg.api_key = buffer.trim_end_matches("\n").to_string();
            confy::store("gpterm", None, cfg.clone())?;
            let file = confy::get_configuration_file_path(APP_NAME, None)?;
            println!("Config updated with new API key and stored here: {:#?}", file)
        }
    }
    Ok(cfg)
}

pub fn delete() -> Result<(), confy::ConfyError> {
    let empty_cfg = GptermConfig::default();
    confy::store(APP_NAME, None, empty_cfg)?;
    Ok(())
}

fn print_no_api_key_info() {
    println!();
    println!("ℹ️ You do not have an API key set. Go to https://platform.openai.com/account/api-keys to make one");
    println!("Copy the API key, and paste it here, and press enter.");
    println!("If you don't like storing API keys in config files, you can set it in environment variable GPTERM_API_KEY");
    println!("API key: ");
}