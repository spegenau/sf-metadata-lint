use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DescribeMetadataObject  {
	#[serde(rename = "childXmlNames")]
	pub child_xml_names: Option<Vec<String>>,
	#[serde(rename = "directoryName")]
	pub directory_name: String,
	#[serde(rename = "inFolder")]
	pub in_folder: bool,
	#[serde(rename = "metaFile")]
	pub meta_file: bool,
	#[serde(rename = "suffix")]
	pub suffix: Option<String>,
	#[serde(rename = "xmlName")]
	pub xml_name: String,
}
