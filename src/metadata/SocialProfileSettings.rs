use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SocialProfileSettings  {
	#[serde(rename = "enableSocialProfiles")]
	pub enable_social_profiles: Option<bool>,
	#[serde(rename = "isFacebookSocialProfilesDisabled")]
	pub is_facebook_social_profiles_disabled: Option<bool>,
	#[serde(rename = "isLinkedInSocialProfilesDisabled")]
	pub is_linked_in_social_profiles_disabled: Option<bool>,
	#[serde(rename = "isTwitterSocialProfilesDisabled")]
	pub is_twitter_social_profiles_disabled: Option<bool>,
	#[serde(rename = "isYouTubeSocialProfilesDisabled")]
	pub is_you_tube_social_profiles_disabled: Option<bool>,
}
