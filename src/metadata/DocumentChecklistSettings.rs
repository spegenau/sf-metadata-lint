use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DocumentChecklistSettings  {
	#[serde(rename = "dciCustomSharing")]
	pub dci_custom_sharing: Option<bool>,
	#[serde(rename = "deleteDCIWithFiles")]
	pub delete_dci_with_files: Option<bool>,
}
