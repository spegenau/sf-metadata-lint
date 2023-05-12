use crate::metadata::CallCenterSection::CallCenterSection;
use crate::metadata::ContactCenterChannel::ContactCenterChannel;
use crate::metadata::VendorCallCenterStatusMap::VendorCallCenterStatusMap;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CallCenter  {
	#[serde(rename = "adapterUrl")]
	pub adapter_url: Option<String>,
	#[serde(rename = "contactCenterChannels")]
	pub contact_center_channels: Option<Vec<ContactCenterChannel>>,
	#[serde(rename = "customSettings")]
	pub custom_settings: Option<String>,
	#[serde(rename = "displayName")]
	pub display_name: String,
	#[serde(rename = "displayNameLabel")]
	pub display_name_label: String,
	#[serde(rename = "internalNameLabel")]
	pub internal_name_label: String,
	#[serde(rename = "sections")]
	pub sections: Option<Vec<CallCenterSection>>,
	#[serde(rename = "vendorCallCenterStatusMaps")]
	pub vendor_call_center_status_maps: Option<Vec<VendorCallCenterStatusMap>>,
	#[serde(rename = "version")]
	pub version: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
