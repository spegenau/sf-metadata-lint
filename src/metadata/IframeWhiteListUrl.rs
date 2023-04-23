use crate::metadata::IFrameWhitelistContext::IFrameWhitelistContext;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IframeWhiteListUrl  {
	#[serde(rename = "context")]
	pub context: IFrameWhitelistContext,
	#[serde(rename = "url")]
	pub url: Option<String>,
}
