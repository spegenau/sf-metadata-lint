use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IdeasSettings  {
	#[serde(rename = "enableChatterProfile")]
	pub enable_chatter_profile: Option<bool>,
	#[serde(rename = "enableHtmlIdea")]
	pub enable_html_idea: Option<bool>,
	#[serde(rename = "enableIdeaMultipleCategory")]
	pub enable_idea_multiple_category: Option<bool>,
	#[serde(rename = "enableIdeaThemes")]
	pub enable_idea_themes: Option<bool>,
	#[serde(rename = "enableIdeas")]
	pub enable_ideas: Option<bool>,
	#[serde(rename = "enableIdeasControllerExtensions")]
	pub enable_ideas_controller_extensions: Option<bool>,
	#[serde(rename = "enableIdeasReputation")]
	pub enable_ideas_reputation: Option<bool>,
	#[serde(rename = "halfLife")]
	pub half_life: Option<f32>,
	#[serde(rename = "ideasProfilePage")]
	pub ideas_profile_page: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
