use crate::metadata::BotNavigationLink::BotNavigationLink;
use crate::metadata::BotNavigationType::BotNavigationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotNavigation  {
	#[serde(rename = "botNavigationLinks")]
	pub bot_navigation_links: Option<Vec<BotNavigationLink>>,
	#[serde(rename = "type")]
	pub _type: BotNavigationType,
}
