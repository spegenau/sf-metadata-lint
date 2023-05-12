use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EnhancedNotesSettings  {
	#[serde(rename = "enableEnhancedNotes")]
	pub enable_enhanced_notes: Option<bool>,
	#[serde(rename = "enableTasksOnEnhancedNotes")]
	pub enable_tasks_on_enhanced_notes: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
