use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkDotComSettings  {
	#[serde(rename = "enableCoachingManagerGroupAccess")]
	pub enable_coaching_manager_group_access: Option<bool>,
	#[serde(rename = "enableGoalManagerGroupAccess")]
	pub enable_goal_manager_group_access: Option<bool>,
	#[serde(rename = "enableProfileSkills")]
	pub enable_profile_skills: Option<bool>,
	#[serde(rename = "enableProfileSkillsAddFeedPost")]
	pub enable_profile_skills_add_feed_post: Option<bool>,
	#[serde(rename = "enableProfileSkillsAutoSuggest")]
	pub enable_profile_skills_auto_suggest: Option<bool>,
	#[serde(rename = "enableProfileSkillsUsePlatform")]
	pub enable_profile_skills_use_platform: Option<bool>,
	#[serde(rename = "enableWorkBadgeDefRestrictPref")]
	pub enable_work_badge_def_restrict_pref: Option<bool>,
	#[serde(rename = "enableWorkCalibration")]
	pub enable_work_calibration: Option<bool>,
	#[serde(rename = "enableWorkCanvasPref")]
	pub enable_work_canvas_pref: Option<bool>,
	#[serde(rename = "enableWorkCertification")]
	pub enable_work_certification: Option<bool>,
	#[serde(rename = "enableWorkCertificationNotification")]
	pub enable_work_certification_notification: Option<bool>,
	#[serde(rename = "enableWorkRewardsPref")]
	pub enable_work_rewards_pref: Option<bool>,
	#[serde(rename = "enableWorkThanksPref")]
	pub enable_work_thanks_pref: Option<bool>,
	#[serde(rename = "enableWorkUseObjectivesForGoals")]
	pub enable_work_use_objectives_for_goals: Option<bool>,
}
