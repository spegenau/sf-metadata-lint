use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MediaAdSalesSettings  {
	#[serde(rename = "enableMediaAdSales")]
	pub enable_media_ad_sales: Option<bool>,
}
