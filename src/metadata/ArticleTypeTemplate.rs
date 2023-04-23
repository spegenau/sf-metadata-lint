use crate::metadata::Channel::Channel;
use crate::metadata::Template::Template;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ArticleTypeTemplate  {
	#[serde(rename = "channel")]
	pub channel: Channel,
	#[serde(rename = "page")]
	pub page: Option<String>,
	#[serde(rename = "template")]
	pub template: Template,
}
