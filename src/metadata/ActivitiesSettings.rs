use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActivitiesSettings  {
	#[serde(rename = "allowUsersToRelateMultipleContactsToTasksAndEvents")]
	pub allow_users_to_relate_multiple_contacts_to_tasks_and_events: Option<bool>,
	#[serde(rename = "autoRelateEventAttendees")]
	pub auto_relate_event_attendees: Option<bool>,
	#[serde(rename = "enableActivityReminders")]
	pub enable_activity_reminders: Option<bool>,
	#[serde(rename = "enableClickCreateEvents")]
	pub enable_click_create_events: Option<bool>,
	#[serde(rename = "enableDragAndDropScheduling")]
	pub enable_drag_and_drop_scheduling: Option<bool>,
	#[serde(rename = "enableEmailTracking")]
	pub enable_email_tracking: Option<bool>,
	#[serde(rename = "enableFlowTaskNotifsViaApex")]
	pub enable_flow_task_notifs_via_apex: Option<bool>,
	#[serde(rename = "enableGroupTasks")]
	pub enable_group_tasks: Option<bool>,
	#[serde(rename = "enableHideChildEventsPreference")]
	pub enable_hide_child_events_preference: Option<bool>,
	#[serde(rename = "enableListViewScheduling")]
	pub enable_list_view_scheduling: Option<bool>,
	#[serde(rename = "enableLogNote")]
	pub enable_log_note: Option<bool>,
	#[serde(rename = "enableMLSingleClientProfile")]
	pub enable_ml_single_client_profile: Option<bool>,
	#[serde(rename = "enableMultidayEvents")]
	pub enable_multiday_events: Option<bool>,
	#[serde(rename = "enableRecurringEvents")]
	pub enable_recurring_events: Option<bool>,
	#[serde(rename = "enableRecurringTasks")]
	pub enable_recurring_tasks: Option<bool>,
	#[serde(rename = "enableRollUpActivToContactsAcct")]
	pub enable_roll_up_activ_to_contacts_acct: Option<bool>,
	#[serde(rename = "enableSidebarCalendarShortcut")]
	pub enable_sidebar_calendar_shortcut: Option<bool>,
	#[serde(rename = "enableSimpleTaskCreateUI")]
	pub enable_simple_task_create_ui: Option<bool>,
	#[serde(rename = "enableTimelineCompDateSort")]
	pub enable_timeline_comp_date_sort: Option<bool>,
	#[serde(rename = "enableUNSTaskDelegatedToNotifications")]
	pub enable_uns_task_delegated_to_notifications: Option<bool>,
	#[serde(rename = "enableUserListViewCalendars")]
	pub enable_user_list_view_calendars: Option<bool>,
	#[serde(rename = "meetingRequestsLogo")]
	pub meeting_requests_logo: Option<String>,
	#[serde(rename = "showCustomLogoMeetingRequests")]
	pub show_custom_logo_meeting_requests: Option<bool>,
	#[serde(rename = "showEventDetailsMultiUserCalendar")]
	pub show_event_details_multi_user_calendar: Option<bool>,
	#[serde(rename = "showHomePageHoverLinksForEvents")]
	pub show_home_page_hover_links_for_events: Option<bool>,
	#[serde(rename = "showMyTasksHoverLinks")]
	pub show_my_tasks_hover_links: Option<bool>,
}