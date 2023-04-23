use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct VendorCallCenterStatusMap  {
	#[serde(rename = "externalStatus")]
	pub external_status: String,
	#[serde(rename = "servicePresenceStatus")]
	pub service_presence_status: String,
}
