use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceAppointmentSettings  {
	#[serde(rename = "appointmentConfirmImg")]
	pub appointment_confirm_img: Option<String>,
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "homeImg")]
	pub home_img: Option<String>,
	#[serde(rename = "logoImg")]
	pub logo_img: Option<String>,
	#[serde(rename = "shouldShowExistingAppointment")]
	pub should_show_existing_appointment: Option<bool>,
	#[serde(rename = "shouldShowNewAppointment")]
	pub should_show_new_appointment: Option<bool>,
}
