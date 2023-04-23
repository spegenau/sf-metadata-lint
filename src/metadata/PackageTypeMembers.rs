use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PackageTypeMembers  {
	#[serde(rename = "members")]
	pub members: Option<Vec<String>>,
	#[serde(rename = "name")]
	pub name: String,
}
