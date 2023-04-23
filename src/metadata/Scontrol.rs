use crate::metadata::Encoding::Encoding;
use crate::metadata::SControlContentSource::SControlContentSource;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Scontrol  {
	#[serde(rename = "contentSource")]
	pub content_source: SControlContentSource,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "encodingKey")]
	pub encoding_key: Encoding,
	#[serde(rename = "fileContent")]
	pub file_content: Option<String>,
	#[serde(rename = "fileName")]
	pub file_name: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "supportsCaching")]
	pub supports_caching: bool,
}
