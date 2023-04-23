use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuickActionSendEmailOptions  {
	#[serde(rename = "defaultEmailTemplateName")]
	pub default_email_template_name: Option<String>,
	#[serde(rename = "ignoreDefaultEmailTemplateSubject")]
	pub ignore_default_email_template_subject: bool,
}
