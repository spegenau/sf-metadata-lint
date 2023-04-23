use crate::metadata::KnowledgeCaseEditor::KnowledgeCaseEditor;
use crate::metadata::KnowledgeCommunitiesSettings::KnowledgeCommunitiesSettings;
use crate::metadata::KnowledgeSitesSettings::KnowledgeSitesSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeCaseSettings  {
	#[serde(rename = "articlePDFCreationProfile")]
	pub article_pdf_creation_profile: Option<String>,
	#[serde(rename = "articlePublicSharingCommunities")]
	pub article_public_sharing_communities: Option<KnowledgeCommunitiesSettings>,
	#[serde(rename = "articlePublicSharingSites")]
	pub article_public_sharing_sites: Option<KnowledgeSitesSettings>,
	#[serde(rename = "articlePublicSharingSitesChatterAnswers")]
	pub article_public_sharing_sites_chatter_answers: Option<KnowledgeSitesSettings>,
	#[serde(rename = "assignTo")]
	pub assign_to: Option<String>,
	#[serde(rename = "customizationClass")]
	pub customization_class: Option<String>,
	#[serde(rename = "defaultContributionArticleType")]
	pub default_contribution_article_type: Option<String>,
	#[serde(rename = "editor")]
	pub editor: Option<KnowledgeCaseEditor>,
	#[serde(rename = "enableArticleCreation")]
	pub enable_article_creation: Option<bool>,
	#[serde(rename = "enableArticlePublicSharingSites")]
	pub enable_article_public_sharing_sites: Option<bool>,
	#[serde(rename = "enableCaseDataCategoryMapping")]
	pub enable_case_data_category_mapping: Option<bool>,
	#[serde(rename = "useProfileForPDFCreation")]
	pub use_profile_for_pdf_creation: Option<bool>,
}
