use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ChatterEmailsMDSettings  {
	#[serde(rename = "enableChatterDigestEmailsApiOnly")]
	pub enable_chatter_digest_emails_api_only: Option<bool>,
	#[serde(rename = "enableChatterEmailAttachment")]
	pub enable_chatter_email_attachment: Option<bool>,
	#[serde(rename = "enableCollaborationEmail")]
	pub enable_collaboration_email: Option<bool>,
	#[serde(rename = "enableDisplayAppDownloadBadges")]
	pub enable_display_app_download_badges: Option<bool>,
	#[serde(rename = "enableEmailReplyToChatter")]
	pub enable_email_reply_to_chatter: Option<bool>,
	#[serde(rename = "enableEmailToChatter")]
	pub enable_email_to_chatter: Option<bool>,
	#[serde(rename = "noQnOwnNotifyOnCaseCmt")]
	pub no_qn_own_notify_on_case_cmt: Option<bool>,
	#[serde(rename = "noQnOwnNotifyOnRep")]
	pub no_qn_own_notify_on_rep: Option<bool>,
	#[serde(rename = "noQnSubNotifyOnBestR")]
	pub no_qn_sub_notify_on_best_r: Option<bool>,
	#[serde(rename = "noQnSubNotifyOnRep")]
	pub no_qn_sub_notify_on_rep: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
