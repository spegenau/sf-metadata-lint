use crate::metadata::FileDownloadBehavior::FileDownloadBehavior;
use crate::metadata::FileType::FileType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FileTypeDispositionAssignmentBean  {
	#[serde(rename = "behavior")]
	pub behavior: FileDownloadBehavior,
	#[serde(rename = "fileType")]
	pub file_type: FileType,
	#[serde(rename = "securityRiskFileType")]
	pub security_risk_file_type: bool,
}
