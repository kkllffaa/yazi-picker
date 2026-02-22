use std::collections::HashMap;
use std::env::args;
use std::error::Error;

use zbus::zvariant::{DeserializeDict, OwnedObjectPath, OwnedValue, Type, Value};
use zbus::{conn, fdo, interface};

use crate::options::{
	Choice, Choices, Filter, FilterRule, FilterRuleType, Filters, PickerMode, PickerRequest,
};
use crate::settings::Settings;

mod fifo;
mod options;
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
#[allow(unused)]
struct PickerOptions {
	/// Label for the accept button with mnemonics (_A => underlined A, __ => _)
	pub accept_label:   Option<String>, // All
	/// Lock parent window
	pub modal:          Option<bool>, // All
	/// List of combo boxes (id, label, choices[(id, label)], pre-selected id from choices)
	pub choices:        Option<Vec<(String, String, Vec<(String, String)>, String)>>, // All
	/// Suggested folder to start picker
	pub current_folder: Option<Vec<u8>>, // All os path

	pub directory: Option<bool>, // Open
	pub multiple:  Option<bool>, // Open, in spec is also in save but its open bug https://github.com/flatpak/xdg-desktop-portal/issues/1877

	/// File filters (name, rules[1 = glob 0 = mime, pattern])
	pub filters:        Option<Vec<(String, Vec<(u32, String)>)>>, // Open, Save
	/// pre-selected filter (name, rules[1 = glob 0 = mime, pattern])
	pub current_filter: Option<(String, Vec<(u32, String)>)>, // Open, Save

	/// Suggested file name
	pub current_name: Option<String>, // Save
	/// Path of an existing file to pre-select
	pub current_file: Option<Vec<u8>>, // Save os path

	/// List of files to save
	pub files: Option<Vec<Vec<Vec<u8>>>>, // SaveMulti os path
}

// return:
//
// all:
// uris: Vec<String> file://path % encoded invalid chars eg. ' '=>%20
// choices: Vec<(String, String)>
//
// Open and Save:
// current_filter (String, Vec<(u32, String)>)
//
// Open
// writable (default false): bool ask user
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
		let args = PickerRequest {
			title,
			accept_label: options.accept_label,
			modal: options.modal.unwrap_or(false),
			choices: make_choices(options.choices),
			current_folder: options.current_folder,
			mode: PickerMode::Open {
				directory: options.directory.unwrap_or(false),
				multiple:  options.multiple.unwrap_or(false),
				filters:   make_filters(options.filters, options.current_filter),
			},
		};
		self.pick(handle, app_id, parent_window, args)
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
		let args = PickerRequest {
			title,
			accept_label: options.accept_label,
			modal: options.modal.unwrap_or(false),
			choices: make_choices(options.choices),
			current_folder: options.current_folder,
			mode: PickerMode::Save {
				current_name: options.current_name,
				current_file: options.current_file,
				filters:      make_filters(options.filters, options.current_filter),
			},
		};
		self.pick(handle, app_id, parent_window, args)
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
		let args = PickerRequest {
			title,
			accept_label: options.accept_label,
			modal: options.modal.unwrap_or(false),
			choices: make_choices(options.choices),
			current_folder: options.current_folder,
			mode: PickerMode::SaveMulti { files: () },
		};
		self.pick(handle, app_id, parent_window, args)
			.await
			.to_fdo_res()
	}
}

fn make_choices(
	choices: Option<Vec<(String, String, Vec<(String, String)>, String)>>,
) -> Vec<Choices> {
	match choices {
		Some(choices) => choices
			.into_iter()
			.map(|f| Choices {
				id:         f.0,
				label:      f.1,
				options:    f
					.2
					.into_iter()
					.map(|f| Choice {
						id:    f.0,
						label: f.1,
					})
					.collect(),
				default_id: f.3,
			})
			.collect(),
		None => vec![],
	}
}
fn make_filters(
	filters: Option<Vec<(String, Vec<(u32, String)>)>>,
	mut current: Option<(String, Vec<(u32, String)>)>,
) -> Filters {
	let mut out = Filters {
		filters:        vec![],
		current_filter: None,
	};
	let filters = match filters {
		Some(f) => f,
		None => {
			if let Some(cur) = current {
				out.filters.push(Filter {
					name:  cur.0.clone(),
					rules: cur
						.1
						.into_iter()
						.map(|f| FilterRule {
							ruletype: if f.0 == 0 {
								FilterRuleType::Glob
							} else {
								FilterRuleType::Mime
							},
							rule:     f.1,
						})
						.collect(),
				});
				out.current_filter = Some(cur.0);
			}
			return out;
		}
	};

	for f in filters {
		if let Some(ref cur) = current {
			if cur.0 == f.0 {
				out.current_filter = Some(current.take().unwrap().0)
			}
		}
		out.filters.push(Filter {
			name:  f.0,
			rules: f
				.1
				.into_iter()
				.map(|f| FilterRule {
					ruletype: if f.0 == 0 {
						FilterRuleType::Mime
					} else {
						FilterRuleType::Glob
					},
					rule:     f.1,
				})
				.collect(),
		});
	}

	out
}
