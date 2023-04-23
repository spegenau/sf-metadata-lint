use crate::metadata::FileTypeDispositionAssignmentBean::FileTypeDispositionAssignmentBean;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FileUploadAndDownloadSecuritySettings  {
	#[serde(rename = "dispositions")]
	pub dispositions: Option<Vec<FileTypeDispositionAssignmentBean>>,
	#[serde(rename = "noHtmlUploadAsAttachment")]
	pub no_html_upload_as_attachment: bool,
}
