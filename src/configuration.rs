use serde::{Serialize, Deserialize};
use std::io;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GptermConfig {
    pub api_key: String,
}

impl ::std::default::Default for GptermConfig {
    fn default() -> Self { Self { api_key: "".into() } }
}

pub fn get() -> Result<GptermConfig, confy::ConfyError> {
    let app_name = "gpterm";
    let mut cfg: GptermConfig = confy::load(app_name, None)?;

    if cfg.api_key.is_empty() {
        println!("No API key is set. Please enter your API key and press enter");
        let mut buffer = String::new();
        let read_result = io::stdin().read_line(&mut buffer);
        if read_result.is_ok() { //TODO Error handling
            cfg.api_key = buffer.trim_end_matches("\n").to_string();
            confy::store("gpterm", None, cfg.clone())?;
            let file = confy::get_configuration_file_path(app_name, None)?;
            println!("Config updated with new API key and stored here: {:#?}", file)
        }
    }
    Ok(cfg)
}