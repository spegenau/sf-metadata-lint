use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppProfileActionOverride  {
	#[serde(rename = "profile")]
	pub profile: String,
}
