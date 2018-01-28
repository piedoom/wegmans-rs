use std::option::Option;
use query::Query;
use query;
use std::string::ToString;
use std::fmt::Display;

/// A Wegmans store
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
	#[serde(rename="StoreNumber")]
	pub store_number: 	i32,
	#[serde(rename="Name")]
	pub name: 			String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct StoreQuery {
	/// Undocumented
	pub status: Option<String>,
	/// Undocumented
	pub division: Option<String>,
	/// Undocumented
	pub zone_name: Option<String>,
	pub network_address: Option<String>,
	pub store_number: Option<i32>,
	// TODO: make enum
	pub location_type: Option<String>
}

impl StoreQuery {
	pub fn new() -> Self {
		StoreQuery::default(
)	}

	pub fn status(mut self, status: &str) {
		self.status = Some(String::from(status));
	}

	pub fn store_number(mut self, store_number: i32) {
		self.store_number = Some(store_number);
	}
}

impl Default for StoreQuery {
	fn default() -> Self {
		StoreQuery {
			status: None,
			division: None,
			zone_name: None,
			network_address: None,
			store_number: None,
			location_type: None
		}
	}
}
