use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ChatterSettings  {
	#[serde(rename = "allowChatterGroupArchiving")]
	pub allow_chatter_group_archiving: Option<bool>,
	#[serde(rename = "allowRecordsInChatterGroup")]
	pub allow_records_in_chatter_group: Option<bool>,
	#[serde(rename = "enableApprovalRequest")]
	pub enable_approval_request: Option<bool>,
	#[serde(rename = "enableCaseFeedRelativeTimestamps")]
	pub enable_case_feed_relative_timestamps: Option<bool>,
	#[serde(rename = "enableChatter")]
	pub enable_chatter: Option<bool>,
	#[serde(rename = "enableChatterEmoticons")]
	pub enable_chatter_emoticons: Option<bool>,
	#[serde(rename = "enableFeedEdit")]
	pub enable_feed_edit: Option<bool>,
	#[serde(rename = "enableFeedPinning")]
	pub enable_feed_pinning: Option<bool>,
	#[serde(rename = "enableFeedsDraftPosts")]
	pub enable_feeds_draft_posts: Option<bool>,
	#[serde(rename = "enableFeedsRichText")]
	pub enable_feeds_rich_text: Option<bool>,
	#[serde(rename = "enableInviteCsnUsers")]
	pub enable_invite_csn_users: Option<bool>,
	#[serde(rename = "enableOutOfOfficeEnabledPref")]
	pub enable_out_of_office_enabled_pref: Option<bool>,
	#[serde(rename = "enableRichLinkPreviewsInFeed")]
	pub enable_rich_link_previews_in_feed: Option<bool>,
	#[serde(rename = "enableTodayRecsInFeed")]
	pub enable_today_recs_in_feed: Option<bool>,
	#[serde(rename = "unlistedGroupsEnabled")]
	pub unlisted_groups_enabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
