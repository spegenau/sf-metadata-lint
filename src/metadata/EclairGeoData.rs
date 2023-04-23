use crate::metadata::EclairMap::EclairMap;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EclairGeoData  {
	#[serde(rename = "maps")]
	pub maps: Option<Vec<EclairMap>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
