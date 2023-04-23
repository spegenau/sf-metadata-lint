use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContentAssetVersion  {
	#[serde(rename = "number")]
	pub number: String,
	#[serde(rename = "pathOnClient")]
	pub path_on_client: String,
	#[serde(rename = "zipEntry")]
	pub zip_entry: Option<String>,
}
