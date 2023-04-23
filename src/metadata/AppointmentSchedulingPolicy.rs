use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppointmentSchedulingPolicy  {
	#[serde(rename = "appointmentAssignmentPolicy")]
	pub appointment_assignment_policy: Option<String>,
	#[serde(rename = "appointmentStartTimeInterval")]
	pub appointment_start_time_interval: String,
	#[serde(rename = "extCalEventHandler")]
	pub ext_cal_event_handler: Option<String>,
	#[serde(rename = "isSvcTerrOpHoursWithShiftsUsed")]
	pub is_svc_terr_op_hours_with_shifts_used: Option<bool>,
	#[serde(rename = "isSvcTerritoryMemberShiftUsed")]
	pub is_svc_territory_member_shift_used: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "shouldCheckExternalCalendar")]
	pub should_check_external_calendar: bool,
	#[serde(rename = "shouldConsiderCalendarEvents")]
	pub should_consider_calendar_events: bool,
	#[serde(rename = "shouldEnforceExcludedResource")]
	pub should_enforce_excluded_resource: bool,
	#[serde(rename = "shouldEnforceRequiredResource")]
	pub should_enforce_required_resource: bool,
	#[serde(rename = "shouldMatchSkill")]
	pub should_match_skill: bool,
	#[serde(rename = "shouldMatchSkillLevel")]
	pub should_match_skill_level: bool,
	#[serde(rename = "shouldRespectVisitingHours")]
	pub should_respect_visiting_hours: bool,
	#[serde(rename = "shouldUsePrimaryMembers")]
	pub should_use_primary_members: bool,
	#[serde(rename = "shouldUseSecondaryMembers")]
	pub should_use_secondary_members: bool,
}
