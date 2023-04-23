use crate::metadata::CodeLocation::CodeLocation;
use crate::metadata::ID::ID;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CodeCoverageResult  {
	#[serde(rename = "dmlInfo")]
	pub dml_info: Option<Vec<CodeLocation>>,
	#[serde(rename = "id")]
	pub id: ID,
	#[serde(rename = "locationsNotCovered")]
	pub locations_not_covered: Option<Vec<CodeLocation>>,
	#[serde(rename = "methodInfo")]
	pub method_info: Option<Vec<CodeLocation>>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "namespace")]
	pub namespace: String,
	#[serde(rename = "numLocations")]
	pub num_locations: i32,
	#[serde(rename = "numLocationsNotCovered")]
	pub num_locations_not_covered: i32,
	#[serde(rename = "soqlInfo")]
	pub soql_info: Option<Vec<CodeLocation>>,
	#[serde(rename = "soslInfo")]
	pub sosl_info: Option<Vec<CodeLocation>>,
	#[serde(rename = "type")]
	pub _type: String,
}
