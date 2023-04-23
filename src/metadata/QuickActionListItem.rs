use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuickActionListItem  {
	#[serde(rename = "quickActionName")]
	pub quick_action_name: String,
}
