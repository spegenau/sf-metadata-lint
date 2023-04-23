use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct GoogleAppsSettings  {
	#[serde(rename = "enableGmailButtons")]
	pub enable_gmail_buttons: Option<bool>,
	#[serde(rename = "enableGmailButtonsAndLinks")]
	pub enable_gmail_buttons_and_links: Option<bool>,
	#[serde(rename = "enableGmailLinks")]
	pub enable_gmail_links: Option<bool>,
	#[serde(rename = "enableGoogleDocs")]
	pub enable_google_docs: Option<bool>,
	#[serde(rename = "enableGoogleDocsTab")]
	pub enable_google_docs_tab: Option<bool>,
	#[serde(rename = "enableGoogleTalk")]
	pub enable_google_talk: Option<bool>,
	#[serde(rename = "googleAppsDomain")]
	pub google_apps_domain: Option<String>,
	#[serde(rename = "googleAppsDomainLinked")]
	pub google_apps_domain_linked: Option<bool>,
	#[serde(rename = "googleAppsDomainValidated")]
	pub google_apps_domain_validated: Option<bool>,
}
