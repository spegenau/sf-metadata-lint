use crate::metadata::DescribeMetadataObject::DescribeMetadataObject;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DescribeMetadataResult  {
	#[serde(rename = "metadataObjects")]
	pub metadata_objects: Option<Vec<DescribeMetadataObject>>,
	#[serde(rename = "organizationNamespace")]
	pub organization_namespace: String,
	#[serde(rename = "partialSaveAllowed")]
	pub partial_save_allowed: bool,
	#[serde(rename = "testRequired")]
	pub test_required: bool,
}
