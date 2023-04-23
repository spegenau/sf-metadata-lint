use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfilePasswordPolicy  {
	#[serde(rename = "forgotPasswordRedirect")]
	pub forgot_password_redirect: Option<bool>,
	#[serde(rename = "lockoutInterval")]
	pub lockout_interval: i32,
	#[serde(rename = "maxLoginAttempts")]
	pub max_login_attempts: i32,
	#[serde(rename = "minimumPasswordLength")]
	pub minimum_password_length: i32,
	#[serde(rename = "minimumPasswordLifetime")]
	pub minimum_password_lifetime: Option<bool>,
	#[serde(rename = "obscure")]
	pub obscure: Option<bool>,
	#[serde(rename = "passwordComplexity")]
	pub password_complexity: i32,
	#[serde(rename = "passwordExpiration")]
	pub password_expiration: i32,
	#[serde(rename = "passwordHistory")]
	pub password_history: i32,
	#[serde(rename = "passwordQuestion")]
	pub password_question: i32,
	#[serde(rename = "profile")]
	pub profile: String,
}
