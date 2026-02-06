use std::collections::HashMap;
use std::env::{current_dir, current_exe, temp_dir};
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use smol::process::Command;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};
use zbus::{conn, interface};

mod fifo;

// --- Configuration Logic ---
#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(default)]
struct Settings {
	terminal_binary: String,
	terminal_args:   Vec<String>,
}
impl Default for Settings {
	fn default() -> Self {
		Self {
			terminal_binary: String::from("kitty"),
			terminal_args:   ["--class=floating", "-e", "sh", "-c"]
				.into_iter()
				.map(String::from)
				.collect(),
		}
	}
}

impl Settings {
	fn load() -> Result<Self, Box<dyn Error>> {
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

// --- D-Bus Service Implementation ---
struct FileChooser {
	settings: Settings,
}

#[interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl FileChooser {
	async fn open_file(
		&self,
		_handle: OwnedObjectPath,
		_app_id: String,
		_parent_window: String,
		title: String,
		options: HashMap<String, Value<'_>>,
	) -> (u32, HashMap<String, OwnedValue>) {
		let error_ret = (1, HashMap::new());

		println!("Request received: {}, options: {:?}", title, options);

		let pick = Path::new("/home/kkllffaa/source/yazi-picker/pick.sh"); // TODO: dont hardcode it and dont rely on $PATH
		if !pick.exists() {
			eprintln!("Error pick.sh not found at {}", pick.display());
			return error_ret;
		}
		let pick = pick.canonicalize().unwrap();

		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos();
		let filename = format!("portal-selection-{}", timestamp);
		let tmp_path = temp_dir().join(filename);
		drop(File::create_new(&tmp_path).unwrap()); // TODO: dont block on file create? idk
		let cmd = format!("{} -o {}", pick.display(), tmp_path.display());

		println!(
			"Launching: {} {:?} '{}'",
			self.settings.terminal_binary, self.settings.terminal_args, cmd
		);

		// ASYNC: This yields the thread so other requests can be processed!
		let output = Command::new(&self.settings.terminal_binary)
			.args(&self.settings.terminal_args)
			.arg(&cmd)
			.output()
			.await; // <--- The magic moment

		match output {
			Ok(out) if !out.status.success() => {
				eprintln!("Picker process exited with error code: {}", out.status);
				return error_ret;
			}
			Err(e) => {
				eprintln!("Failed to launch picker process: {}", e);
				return error_ret;
			}
			Ok(_) => {}
		}

		// TODO: dont block
		let mut selection = String::new();
		match File::open(&tmp_path) {
			Ok(mut f) => {
				if let Err(e) = f.read_to_string(&mut selection) {
					eprintln!("Failed to read file: {}", e);
					return error_ret;
				}
			}
			Err(e) => {
				eprintln!("Failed to open file: {}", e);
				return error_ret;
			}
		}
		fs::remove_file(tmp_path).unwrap();

		let selection = selection.trim();
		if selection.is_empty() {
			eprintln!("Selection cancelled or empty");
			return error_ret;
		}

		let uris: Vec<String> = selection
			.lines()
			.map(|l| l.trim())
			.filter(|l| !l.is_empty())
			.map(|p| format!("file://{}", p))
			.collect();

		println!("User selected: {}", selection);
		println!("User selected uris: {:?}", uris);

		let mut response = HashMap::new();

		response.insert(
			"uris".to_string(),
			Value::from(uris).try_into_owned().unwrap(),
		);
		(0, response) // 0 = Success
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let chooser = FileChooser {
		settings: Settings::load()?,
	};

	println!("settings: {:?}", &chooser.settings);

	smol::block_on(async {
		let _conn = conn::Builder::session()?
			.name("org.freedesktop.impl.portal.desktop.rust_backend")?
			.serve_at("/org/freedesktop/portal/desktop", chooser)?
			.build()
			.await?;

		println!("Service running. Press Ctrl+C to stop.");

		std::future::pending::<()>().await;
		Ok(())
	})
}
