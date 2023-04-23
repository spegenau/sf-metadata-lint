use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OpportunityListFieldsUnselectedSettings  {
	#[serde(rename = "field")]
	pub field: Option<Vec<String>>,
}
