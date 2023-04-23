use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCategoryItems  {
	#[serde(rename = "flow")]
	pub flow: String,
}
