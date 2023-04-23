use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TrailheadSettings  {
	#[serde(rename = "enableConfettiEffect")]
	pub enable_confetti_effect: Option<bool>,
	#[serde(rename = "enableMyTrailheadPref")]
	pub enable_my_trailhead_pref: Option<bool>,
	#[serde(rename = "enableTrailheadInLexTerms")]
	pub enable_trailhead_in_lex_terms: Option<bool>,
}
