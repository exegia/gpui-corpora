#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct NotificationRequest {
    pub title: SharedString,
    pub body: SharedString,
    pub play_sound: bool,
    pub thread_id: Option<String>,
    pub category: Option<String>,
    pub prefer_native: bool,
    pub importance: NotificationImportance,
    /// Force the in-app feedback toast even on native success: a D-Bus `Ok`
    /// only means accepted — GNOME suppresses banners under DND/mute/fullscreen.
    pub force_in_app_feedback: bool,
}

impl NotificationRequest {
    pub fn foreground(title: impl Into<SharedString>, body: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            play_sound: true,
            thread_id: None,
            category: None,
            prefer_native: true,
            importance: NotificationImportance::ForegroundOnly,
            force_in_app_feedback: false,
        }
    }

    /// A user-initiated "send a test notification": `Critical` urgency (the
    /// only level piercing GNOME's DND/mute/fullscreen gates) + forced feedback.
    pub fn test_notification(
        title: impl Into<SharedString>,
        body: impl Into<SharedString>,
    ) -> Self {
        let mut request = Self::foreground(title, body);
        request.importance = NotificationImportance::BackgroundWorthy;
        request.force_in_app_feedback = true;
        request
    }

    pub fn action_buttons(title: impl Into<SharedString>, body: impl Into<SharedString>) -> Self {
        let mut request = Self::foreground(title, body);
        request.category = Some(CATEGORY_ACTIONS.to_string());
        request.thread_id = Some("settings-actions".to_string());
        request
    }

    pub fn reply(title: impl Into<SharedString>, body: impl Into<SharedString>) -> Self {
        let mut request = Self::foreground(title, body);
        request.category = Some(CATEGORY_REPLY.to_string());
        request.thread_id = Some("settings-reply".to_string());
        request
    }

    pub fn background_worthy(
        title: impl Into<SharedString>,
        body: impl Into<SharedString>,
    ) -> Self {
        let mut request = Self::foreground(title, body);
        request.importance = NotificationImportance::BackgroundWorthy;
        request.thread_id = Some("settings-background-worthy".to_string());
        request
    }
}
