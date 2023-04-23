use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ChatterAnswersSettings  {
	#[serde(rename = "emailFollowersOnBestAnswer")]
	pub email_followers_on_best_answer: Option<bool>,
	#[serde(rename = "emailFollowersOnReply")]
	pub email_followers_on_reply: Option<bool>,
	#[serde(rename = "emailOwnerOnPrivateReply")]
	pub email_owner_on_private_reply: Option<bool>,
	#[serde(rename = "emailOwnerOnReply")]
	pub email_owner_on_reply: Option<bool>,
	#[serde(rename = "enableAnswerViaEmail")]
	pub enable_answer_via_email: Option<bool>,
	#[serde(rename = "enableChatterAnswers")]
	pub enable_chatter_answers: bool,
	#[serde(rename = "enableFacebookSSO")]
	pub enable_facebook_sso: Option<bool>,
	#[serde(rename = "enableInlinePublisher")]
	pub enable_inline_publisher: Option<bool>,
	#[serde(rename = "enableReputation")]
	pub enable_reputation: Option<bool>,
	#[serde(rename = "enableRichTextEditor")]
	pub enable_rich_text_editor: Option<bool>,
	#[serde(rename = "facebookAuthProvider")]
	pub facebook_auth_provider: Option<String>,
	#[serde(rename = "showInPortals")]
	pub show_in_portals: Option<bool>,
}
