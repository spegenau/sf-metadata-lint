use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppActionOverride  {
	#[serde(rename = "pageOrSobjectType")]
	pub page_or_sobject_type: String,
}
