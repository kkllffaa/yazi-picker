use std::collections::HashMap;
use std::error::Error;

use zbus::zvariant::{DeserializeDict, OwnedObjectPath, OwnedValue, Type, Value};
use zbus::{conn, fdo, interface};

use crate::settings::Settings;

mod fifo;
mod picker;
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
	// let conff = include_str!("../default-config.toml");

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

struct FileChooser {
	settings: Settings,
}

pub enum PickerResult {
	Success(Vec<String>),
	Cancelled,
	Failure,
	ProtocolError(String),
}
impl PickerResult {
	fn to_fdo_res(self) -> fdo::Result<(u32, HashMap<String, OwnedValue>)> {
		match self {
			PickerResult::Success(uris) => {
				let mut map = HashMap::new();
				map.insert(
					"uris".to_string(),
					Value::from(uris).try_into_owned().unwrap(),
				);
				Ok((0, map))
			}
			PickerResult::Cancelled => Ok((1, HashMap::new())),
			PickerResult::Failure => Ok((2, HashMap::new())),
			PickerResult::ProtocolError(msg) => Err(fdo::Error::InvalidArgs(msg)), // TODO: more error types
		}
	}
}

#[derive(Debug, Default, Type, DeserializeDict)]
#[zvariant(signature = "dict")]
// #[serde(deny_unknown_fields)]
struct PickerOptions {
	pub _accept_label: Option<String>,
	pub _modal:        Option<bool>,
	pub _multiple:     Option<bool>,
	pub _directory:    Option<bool>,
	pub _filters:      Option<Vec<(String, Vec<(u32, String)>)>>,
	pub _current_name: Option<String>,
}

#[interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl FileChooser {
	async fn open_file(
		&self,
		handle: OwnedObjectPath,
		app_id: String,
		parent_window: String,
		title: String,
		options: PickerOptions,
	) -> fdo::Result<(u32, HashMap<String, OwnedValue>)> {
		self.pick(handle, app_id, parent_window, title, options)
			.await
			.to_fdo_res()
	}

	async fn save_file(
		&self,
		handle: OwnedObjectPath,
		app_id: String,
		parent_window: String,
		title: String,
		options: PickerOptions,
	) -> fdo::Result<(u32, HashMap<String, OwnedValue>)> {
		self.pick(handle, app_id, parent_window, title, options)
			.await
			.to_fdo_res()
	}

	async fn save_files(
		&self,
		handle: OwnedObjectPath,
		app_id: String,
		parent_window: String,
		title: String,
		options: PickerOptions,
	) -> fdo::Result<(u32, HashMap<String, OwnedValue>)> {
		self.pick(handle, app_id, parent_window, title, options)
			.await
			.to_fdo_res()
	}
}
