use crate::metadata::AssignmentPolicyType::AssignmentPolicyType;
use crate::metadata::PolicyApplicableDuration::PolicyApplicableDuration;
use crate::metadata::UtilizationFactor::UtilizationFactor;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppointmentAssignmentPolicy  {
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "policyApplicableDuration")]
	pub policy_applicable_duration: PolicyApplicableDuration,
	#[serde(rename = "policyType")]
	pub policy_type: AssignmentPolicyType,
	#[serde(rename = "utilizationFactor")]
	pub utilization_factor: UtilizationFactor,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
