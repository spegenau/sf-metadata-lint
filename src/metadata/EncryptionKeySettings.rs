use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EncryptionKeySettings  {
	#[serde(rename = "canOptOutOfDerivationWithBYOK")]
	pub can_opt_out_of_derivation_with_byok: Option<bool>,
	#[serde(rename = "enableCacheOnlyKeys")]
	pub enable_cache_only_keys: Option<bool>,
	#[serde(rename = "enableReplayDetection")]
	pub enable_replay_detection: Option<bool>,
}
