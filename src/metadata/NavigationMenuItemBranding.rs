use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NavigationMenuItemBranding  {
	#[serde(rename = "tileImage")]
	pub tile_image: Option<String>,
}
