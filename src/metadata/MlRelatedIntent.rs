use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MlRelatedIntent  {
	#[serde(rename = "relatedMlIntent")]
	pub related_ml_intent: String,
}
