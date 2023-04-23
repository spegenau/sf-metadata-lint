use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContentSettings  {
	#[serde(rename = "enableCMSC2CConnections")]
	pub enable_cmsc_2_c_connections: Option<bool>,
	#[serde(rename = "enableChatterFileLink")]
	pub enable_chatter_file_link: Option<bool>,
	#[serde(rename = "enableContent")]
	pub enable_content: Option<bool>,
	#[serde(rename = "enableContentAutoAssign")]
	pub enable_content_auto_assign: Option<bool>,
	#[serde(rename = "enableContentDistForPortalUsers")]
	pub enable_content_dist_for_portal_users: Option<bool>,
	#[serde(rename = "enableContentDistPwOptionsBit1")]
	pub enable_content_dist_pw_options_bit_1: Option<bool>,
	#[serde(rename = "enableContentDistPwOptionsBit2")]
	pub enable_content_dist_pw_options_bit_2: Option<bool>,
	#[serde(rename = "enableContentDistribution")]
	pub enable_content_distribution: Option<bool>,
	#[serde(rename = "enableContentSupportMultiLanguage")]
	pub enable_content_support_multi_language: Option<bool>,
	#[serde(rename = "enableContentWorkspaceAccess")]
	pub enable_content_workspace_access: Option<bool>,
	#[serde(rename = "enableDeleteFileInContentPacks")]
	pub enable_delete_file_in_content_packs: Option<bool>,
	#[serde(rename = "enableFileShareSetByRecord")]
	pub enable_file_share_set_by_record: Option<bool>,
	#[serde(rename = "enableFilesUsrShareNetRestricted")]
	pub enable_files_usr_share_net_restricted: Option<bool>,
	#[serde(rename = "enableJPGPreviews")]
	pub enable_jpg_previews: Option<bool>,
	#[serde(rename = "enableLibraryManagedFiles")]
	pub enable_library_managed_files: Option<bool>,
	#[serde(rename = "enableShowChatterFilesInContent")]
	pub enable_show_chatter_files_in_content: Option<bool>,
	#[serde(rename = "enableSiteGuestUserToUploadFiles")]
	pub enable_site_guest_user_to_upload_files: Option<bool>,
	#[serde(rename = "enableUploadFilesOnAttachments")]
	pub enable_upload_files_on_attachments: Option<bool>,
	#[serde(rename = "setValidContentTypeForAtchDocDownload")]
	pub set_valid_content_type_for_atch_doc_download: Option<bool>,
	#[serde(rename = "skipContentAssetTriggers")]
	pub skip_content_asset_triggers: Option<bool>,
	#[serde(rename = "skipContentAssetTriggersOnDeploy")]
	pub skip_content_asset_triggers_on_deploy: Option<bool>,
}
