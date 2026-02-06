use std::collections::HashMap;
use std::env::temp_dir;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use config::Config;
use smol::process::Command;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};
use zbus::{conn, interface};

mod fifo;

// --- Configuration Logic ---
#[derive(serde::Deserialize)]
struct Settings {
	terminal_binary: String,
	terminal_args:   Vec<String>,
}

impl Settings {
	fn load() -> Result<Self, Box<dyn Error>> {
		// 1. Start with Config File
		let mut builder = Config::builder()
			.set_default("terminal_binary", "kitty")?
			.set_default("terminal_args", vec!["--class=floating", "-e", "sh", "-c"])?;

		if let Some(config_dir) = dirs::config_dir() {
			let config_path = config_dir.join("yazi-picker").join("config");
			builder = builder.add_source(config::File::from(config_path).required(false));
		}
		builder = builder.add_source(config::File::with_name("config").required(false));

		// 2. Override with TERMINAL env var (Convention)
		// if let Ok(term) = env::var("TERMINAL") {
		// 	builder = builder.set_override("terminal_binary", term)?;
		// }

		Ok(builder.build()?.try_deserialize()?)
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
