use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowTaskTranslation  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "subject")]
	pub subject: Option<String>,
}
