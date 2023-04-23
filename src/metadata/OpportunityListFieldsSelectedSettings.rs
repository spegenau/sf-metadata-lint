use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OpportunityListFieldsSelectedSettings  {
	#[serde(rename = "field")]
	pub field: Option<Vec<String>>,
}
