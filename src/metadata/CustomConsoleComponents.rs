use crate::metadata::PrimaryTabComponents::PrimaryTabComponents;
use crate::metadata::SubtabComponents::SubtabComponents;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomConsoleComponents  {
	#[serde(rename = "primaryTabComponents")]
	pub primary_tab_components: Option<PrimaryTabComponents>,
	#[serde(rename = "subtabComponents")]
	pub subtab_components: Option<SubtabComponents>,
}
