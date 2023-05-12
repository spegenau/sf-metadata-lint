use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowAction  {
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
