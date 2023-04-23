use crate::metadata::IframeWhiteListUrl::IframeWhiteListUrl;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IframeWhiteListUrlSettings  {
	#[serde(rename = "iframeWhiteListUrls")]
	pub iframe_white_list_urls: Option<Vec<IframeWhiteListUrl>>,
}
