use crate::metadata::IpRange::IpRange;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NetworkAccess  {
	#[serde(rename = "ipRanges")]
	pub ip_ranges: Option<Vec<IpRange>>,
}
