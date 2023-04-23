use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MailMergeSettings  {
	#[serde(rename = "enableExtendedMailMerge")]
	pub enable_extended_mail_merge: Option<bool>,
	#[serde(rename = "saveMailMergeDocsAsSalesforceDocs")]
	pub save_mail_merge_docs_as_salesforce_docs: Option<bool>,
}
