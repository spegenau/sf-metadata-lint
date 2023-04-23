use crate::metadata::Complexity::Complexity;
use crate::metadata::Expiration::Expiration;
use crate::metadata::LockoutInterval::LockoutInterval;
use crate::metadata::MaxLoginAttempts::MaxLoginAttempts;
use crate::metadata::QuestionRestriction::QuestionRestriction;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PasswordPolicies  {
	#[serde(rename = "apiOnlyUserHomePageURL")]
	pub api_only_user_home_page_url: Option<String>,
	#[serde(rename = "complexity")]
	pub complexity: Option<Complexity>,
	#[serde(rename = "expiration")]
	pub expiration: Option<Expiration>,
	#[serde(rename = "historyRestriction")]
	pub history_restriction: Option<String>,
	#[serde(rename = "lockoutInterval")]
	pub lockout_interval: Option<LockoutInterval>,
	#[serde(rename = "maxLoginAttempts")]
	pub max_login_attempts: Option<MaxLoginAttempts>,
	#[serde(rename = "minimumPasswordLength")]
	pub minimum_password_length: Option<String>,
	#[serde(rename = "minimumPasswordLifetime")]
	pub minimum_password_lifetime: Option<bool>,
	#[serde(rename = "obscureSecretAnswer")]
	pub obscure_secret_answer: Option<bool>,
	#[serde(rename = "passwordAssistanceMessage")]
	pub password_assistance_message: Option<String>,
	#[serde(rename = "passwordAssistanceURL")]
	pub password_assistance_url: Option<String>,
	#[serde(rename = "questionRestriction")]
	pub question_restriction: Option<QuestionRestriction>,
}
