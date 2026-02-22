use std::env::temp_dir;
use std::fs::{self, File, create_dir};
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use smol::process::Command;
use zbus::zvariant::OwnedObjectPath;

use crate::FileChooser;
use crate::PickerResult::{self, *};
use crate::options::*;

impl FileChooser {
	pub async fn pick(
		&self,
		_handle: OwnedObjectPath,
		_app_id: String,
		_parent_window: String,
		args: PickerRequest,
	) -> PickerResult {
		println!("args: {}", serde_json::to_string_pretty(&args).unwrap());

		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos();
		let tmp_dir_name = format!("portal-selection-{}", timestamp);
		let tmp_dir_path = temp_dir().join(tmp_dir_name);
		let tmp_request_path = tmp_dir_path.join("request.json");
		let tmp_response_path = tmp_dir_path.join("response.json");

		create_dir(&tmp_dir_path).unwrap();

		// TODO: dont block on file create? idk
		match File::create_new(&tmp_request_path) {
			Ok(f) => {
				serde_json::to_writer(f, &args).unwrap();
			}
			Err(_) => return Failure,
		}

		let cmd = format!(
			"../pick.sh -j -i {} -o {}", // TODO
			tmp_request_path.display(),
			tmp_response_path.display()
		);

		println!(
			"Launching: {} {:?} '{}'",
			self.settings.terminal_binary, self.settings.terminal_args, cmd
		);

		let output = Command::new(&self.settings.terminal_binary)
			.args(&self.settings.terminal_args)
			.arg(&cmd)
			.output()
			.await;

		match output {
			Ok(out) if !out.status.success() => {
				eprintln!("Picker process exited with error code: {}", out.status);
				return Failure;
			}
			Err(e) => {
				eprintln!("Failed to launch picker process: {}", e);
				return Failure;
			}
			Ok(_) => {}
		}

		// TODO: dont block
		let mut selection = String::new();
		match File::open(&tmp_response_path) {
			Ok(mut f) => {
				if let Err(e) = f.read_to_string(&mut selection) {
					eprintln!("Failed to read file: {}", e);
					return Failure;
				}
			}
			Err(e) => {
				eprintln!("Failed to open file: {}", e);
				return Failure;
			}
		}
		fs::remove_dir_all(tmp_dir_path).unwrap();

		let selection: PickerResponse = serde_json::from_str(&selection).unwrap();
		if selection.files.is_empty() {
			eprintln!("Selection cancelled or empty");
			return Failure;
		}

		let uris: Vec<String> = selection
			.files
			.iter()
			.map(|l| l.trim())
			.filter(|l| !l.is_empty())
			.map(|p| format!("file://{}", p))
			.collect();

		println!("Response: {:?}", selection);
		println!("User selected uris: {:?}", uris);

		Success(uris)
	}
}
