use crate::metadata::ArticleTypeTemplate::ArticleTypeTemplate;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ArticleTypeChannelDisplay  {
	#[serde(rename = "articleTypeTemplates")]
	pub article_type_templates: Option<Vec<ArticleTypeTemplate>>,
}
