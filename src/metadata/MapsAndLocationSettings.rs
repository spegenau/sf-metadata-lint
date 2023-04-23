use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MapsAndLocationSettings  {
	#[serde(rename = "enableAddressAutoComplete")]
	pub enable_address_auto_complete: Option<bool>,
	#[serde(rename = "enableMapsAndLocation")]
	pub enable_maps_and_location: Option<bool>,
}
