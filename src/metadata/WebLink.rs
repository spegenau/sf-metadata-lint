use crate::metadata::Encoding::Encoding;
use crate::metadata::WebLinkAvailability::WebLinkAvailability;
use crate::metadata::WebLinkDisplayType::WebLinkDisplayType;
use crate::metadata::WebLinkPosition::WebLinkPosition;
use crate::metadata::WebLinkType::WebLinkType;
use crate::metadata::WebLinkWindowType::WebLinkWindowType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WebLink  {
	#[serde(rename = "availability")]
	pub availability: WebLinkAvailability,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "displayType")]
	pub display_type: WebLinkDisplayType,
	#[serde(rename = "encodingKey")]
	pub encoding_key: Option<Encoding>,
	#[serde(rename = "hasMenubar")]
	pub has_menubar: Option<bool>,
	#[serde(rename = "hasScrollbars")]
	pub has_scrollbars: Option<bool>,
	#[serde(rename = "hasToolbar")]
	pub has_toolbar: Option<bool>,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "isResizable")]
	pub is_resizable: Option<bool>,
	#[serde(rename = "linkType")]
	pub link_type: WebLinkType,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "openType")]
	pub open_type: WebLinkWindowType,
	#[serde(rename = "page")]
	pub page: Option<String>,
	#[serde(rename = "position")]
	pub position: Option<WebLinkPosition>,
	#[serde(rename = "protected")]
	pub protected: bool,
	#[serde(rename = "requireRowSelection")]
	pub require_row_selection: Option<bool>,
	#[serde(rename = "scontrol")]
	pub scontrol: Option<String>,
	#[serde(rename = "showsLocation")]
	pub shows_location: Option<bool>,
	#[serde(rename = "showsStatus")]
	pub shows_status: Option<bool>,
	#[serde(rename = "url")]
	pub url: Option<String>,
	#[serde(rename = "width")]
	pub width: Option<i32>,
}
