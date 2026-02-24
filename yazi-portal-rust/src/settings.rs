use std::env::{current_dir, current_exe};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub(crate) const CONFIG_LOCATION: &str = "yazi-picker";

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
	pub terminal_binary: String,
	pub terminal_args:   Vec<String>,
}

impl Config {
	pub fn load() -> Result<Self, Box<dyn Error>> {
		let defaults = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/default-config.toml"));
		let defaults: Config = toml::from_str(defaults).unwrap();

		let mut conf_file = None;
		if let Some(cf) = dirs::config_dir().map(|f| f.join(CONFIG_LOCATION).join("config.toml")) {
			if cf.exists() {
				conf_file = Some(cf);
			}
		}
		{
			let local_cf = PathBuf::from("config.toml");
			let _local_cf = current_dir()?.join("config.toml");
			let _local_cf = current_exe()?.parent().unwrap().join("config.toml");
			if local_cf.exists() {
				conf_file = Some(local_cf)
			}
		}
		let config: Config = if let Some(ref cf) = conf_file {
			println!("loading config from: {}", cf.display());
			toml::from_str(&fs::read_to_string(cf)?)?
		} else {
			println!("using default config");
			defaults
		};

		// 2. Override with TERMINAL env var (Convention)
		// if let Ok(term) = env::var("TERMINAL") {
		// 	builder = builder.set_override("terminal_binary", term)?;
		// }

		Ok(config)
	}
}
