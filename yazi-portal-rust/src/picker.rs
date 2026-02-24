use std::env::temp_dir;
use std::fs::{File, create_dir, remove_dir_all};
use std::io::Read;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use smol::process::Command;
use zbus::zvariant::OwnedObjectPath;

use crate::FileChooser;
use crate::PickerResult::{self, *};
use crate::options::*;
use crate::settings::CONFIG_LOCATION;

impl FileChooser {
	pub async fn pick(
		&self,
		_handle: OwnedObjectPath,
		_app_id: String,
		_parent_window: String,
		args: PickerRequest,
	) -> PickerResult {
		println!("request: {}", serde_json::to_string_pretty(&args).unwrap());

		let tmp_dir = TempDir::new();

		let tmp_request_path = tmp_dir.0.join("request.json");
		let tmp_response_path = tmp_dir.0.join("response.json");

		// TODO: dont block on file create? idk
		match File::create_new(&tmp_request_path) {
			Ok(f) => {
				serde_json::to_writer(f, &args).unwrap();
			}
			Err(_) => return Failure,
		}

		let conf_dir = dirs::config_dir().unwrap().join(CONFIG_LOCATION);

		let ref raw_args = self.settings.terminal_args;
		let mut cmd_args = Vec::new();
		for arg in raw_args {
			let r = replace_all(arg, |l| match l {
				'i' => tmp_request_path.to_str().unwrap(),
				'o' => tmp_response_path.to_str().unwrap(),
				't' => tmp_dir.0.to_str().unwrap(),
				'c' => conf_dir.to_str().unwrap(),
				_ => panic!(),
			});
			cmd_args.push(r);
		}

		println!(
			"Launching: {} {:?}",
			self.settings.terminal_binary, cmd_args
		);

		let output = Command::new(&self.settings.terminal_binary)
			.args(&cmd_args)
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

struct TempDir(PathBuf);
impl Drop for TempDir {
	fn drop(&mut self) { remove_dir_all(&self.0).unwrap() }
}
impl TempDir {
	fn new() -> Self {
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos();
		let tmp_dir_name = format!("portal-selection-{}", timestamp);
		let tmp_dir_path = temp_dir().join(tmp_dir_name);
		create_dir(&tmp_dir_path).unwrap();
		Self(tmp_dir_path)
	}
}
fn replace_all<'a, F: Fn(char) -> &'a str>(mut input: &str, aaa: F) -> String {
	let mut result = String::with_capacity(input.len());

	while let Some(start) = input.find("{") {
		result.push_str(&input[..start]);
		let reminder = &input[start..];

		if reminder.len() >= 3
			&& reminder.as_bytes()[1].is_ascii_alphabetic()
			&& reminder.as_bytes()[2] == b'}'
		{
			let letter = input.as_bytes()[start + 1] as char;

			let replace = aaa(letter);

			result.push_str(replace);
			input = &reminder[3..];
		} else {
			result.push('{');
			input = &reminder[1..];
		}
	}
	result.push_str(input);
	result
}
