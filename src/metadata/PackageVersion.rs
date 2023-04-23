use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PackageVersion  {
	#[serde(rename = "majorNumber")]
	pub major_number: i32,
	#[serde(rename = "minorNumber")]
	pub minor_number: i32,
	#[serde(rename = "namespace")]
	pub namespace: String,
}
