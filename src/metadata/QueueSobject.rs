use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QueueSobject  {
	#[serde(rename = "sobjectType")]
	pub sobject_type: String,
}
