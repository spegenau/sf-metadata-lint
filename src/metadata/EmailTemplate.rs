use crate::metadata::Attachment::Attachment;
use crate::metadata::EmailTemplateStyle::EmailTemplateStyle;
use crate::metadata::EmailTemplateType::EmailTemplateType;
use crate::metadata::EmailTemplateUiType::EmailTemplateUiType;
use crate::metadata::Encoding::Encoding;
use crate::metadata::PackageVersion::PackageVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailTemplate  {
	#[serde(rename = "apiVersion")]
	pub api_version: Option<f32>,
	#[serde(rename = "attachedDocuments")]
	pub attached_documents: Option<Vec<String>>,
	#[serde(rename = "attachments")]
	pub attachments: Option<Vec<Attachment>>,
	#[serde(rename = "available")]
	pub available: bool,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "encodingKey")]
	pub encoding_key: Encoding,
	#[serde(rename = "letterhead")]
	pub letterhead: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "packageVersions")]
	pub package_versions: Option<Vec<PackageVersion>>,
	#[serde(rename = "pageDevName")]
	pub page_dev_name: Option<String>,
	#[serde(rename = "relatedEntityType")]
	pub related_entity_type: Option<String>,
	#[serde(rename = "style")]
	pub style: EmailTemplateStyle,
	#[serde(rename = "subject")]
	pub subject: Option<String>,
	#[serde(rename = "textOnly")]
	pub text_only: Option<String>,
	#[serde(rename = "type")]
	pub _type: EmailTemplateType,
	#[serde(rename = "uiType")]
	pub ui_type: Option<EmailTemplateUiType>,
}
