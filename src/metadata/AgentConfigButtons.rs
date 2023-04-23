use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AgentConfigButtons  {
	#[serde(rename = "button")]
	pub button: Option<Vec<String>>,
}
