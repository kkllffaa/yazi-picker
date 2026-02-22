use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

use serde::ser::{Error, SerializeMap};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize)]
pub(crate) enum FilterRuleType {
	Mime,
	Glob,
}
#[derive(Serialize)]
pub(crate) struct FilterRule {
	pub ruletype: FilterRuleType,
	pub rule:     String,
}
#[derive(Serialize)]
pub(crate) struct Filter {
	pub name:  String,
	pub rules: Vec<FilterRule>,
}
impl MapEntry for Filter {
	type Key = String;
	type Value = Vec<FilterRule>;

	fn get_key(&self) -> &Self::Key { &self.name }

	fn get_value(&self) -> &Self::Value { &self.rules }
}

#[derive(Serialize)]
pub(crate) struct Filters {
	#[serde(serialize_with = "serialize_slice_as_map")]
	pub filters:        Vec<Filter>,
	pub current_filter: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Choice {
	pub id:    String,
	pub label: String,
}
impl MapEntry for Choice {
	type Key = String;
	type Value = String;

	fn get_key(&self) -> &Self::Key { &self.id }

	fn get_value(&self) -> &Self::Value { &self.label }
}
#[derive(Serialize)]
pub(crate) struct Choices {
	#[serde(skip)]
	pub id:         String,
	pub label:      String,
	#[serde(serialize_with = "serialize_slice_as_map")]
	pub options:    Vec<Choice>,
	pub default_id: String,
}
impl MapEntry for Choices {
	type Key = String;
	type Value = Self;

	fn get_key(&self) -> &Self::Key { &self.id }

	fn get_value(&self) -> &Self::Value { &self }
}

#[derive(Serialize)]
pub(crate) struct PickerRequest {
	pub title:          String,
	pub accept_label:   Option<String>,
	pub modal:          bool,
	#[serde(serialize_with = "serialize_slice_as_map")]
	pub choices:        Vec<Choices>,
	pub current_folder: Option<Vec<u8>>,
	pub mode:           PickerMode,
}
#[derive(Serialize)]
#[serde(tag = "id")]
pub(crate) enum PickerMode {
	Open {
		directory: bool,
		multiple:  bool,
		#[serde(flatten)]
		filters:   Filters,
	},
	Save {
		current_name: Option<String>,
		current_file: Option<Vec<u8>>,
		#[serde(flatten)]
		filters:      Filters,
	},
	SaveMulti {
		files: (),
	},
}
#[derive(Deserialize, Debug)]
pub(crate) struct PickerResponse {
	pub files: Vec<String>,
}

pub trait MapEntry {
	type Key: Serialize + Hash + Eq + Display;
	type Value: Serialize;

	fn get_key(&self) -> &Self::Key;
	fn get_value(&self) -> &Self::Value;
}
pub fn serialize_slice_as_map<T, S>(slice: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
	T: MapEntry,
	S: Serializer,
{
	let mut map = serializer.serialize_map(Some(slice.len()))?;
	let mut seen_keys = HashSet::with_capacity(slice.len());

	for item in slice {
		let key = item.get_key();

		if !seen_keys.insert(key) {
			return Err(S::Error::custom(format!("Duplicate key found: {}", key)));
		}

		map.serialize_entry(key, item.get_value())?;
	}

	map.end()
}
