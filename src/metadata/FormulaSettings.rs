use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FormulaSettings  {
	#[serde(rename = "enableDSTAwareDatevalue")]
	pub enable_dst_aware_datevalue: Option<bool>,
}
