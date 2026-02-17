use std::env::{current_dir, current_exe};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// --- Configuration Logic ---
#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub(crate) struct Settings {
	pub terminal_binary: String,
	pub terminal_args:   Vec<String>,
}
impl Default for Settings {
	fn default() -> Self {
		Self {
			terminal_binary: String::from("kitty"),
			terminal_args:   ["--class=yazi-picker", "-e", "sh", "-c"]
				.into_iter()
				.map(String::from)
				.collect(),
		}
	}
}

impl Settings {
	pub fn load() -> Result<Self, Box<dyn Error>> {
		let mut conf_file = None;
		if let Some(cf) = dirs::config_dir().map(|f| f.join("yazi-picker").join("config.toml")) {
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
		let settings: Settings = if let Some(cf) = conf_file {
			println!("loading config from: {}", cf.display());
			toml::from_str(&fs::read_to_string(cf)?)?
		} else {
			println!("using default config");
			Default::default()
		};

		// 2. Override with TERMINAL env var (Convention)
		// if let Ok(term) = env::var("TERMINAL") {
		// 	builder = builder.set_override("terminal_binary", term)?;
		// }

		Ok(settings)
	}
}
