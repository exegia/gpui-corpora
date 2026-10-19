#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "macos")]
pub(crate) fn categories() -> Vec<user_notify::NotificationCategory> {
    vec![
        user_notify::NotificationCategory {
            identifier: CATEGORY_ACTIONS.to_string(),
            actions: vec![
                user_notify::NotificationCategoryAction::Action {
                    identifier: ACTION_OPEN.to_string(),
                    title: crate::i18n::localize("notifications_action_open"),
                },
                user_notify::NotificationCategoryAction::Action {
                    identifier: ACTION_SNOOZE.to_string(),
                    title: crate::i18n::localize("notifications_action_snooze"),
                },
            ],
        },
        user_notify::NotificationCategory {
            identifier: CATEGORY_REPLY.to_string(),
            actions: vec![user_notify::NotificationCategoryAction::TextInputAction {
                identifier: ACTION_REPLY.to_string(),
                title: crate::i18n::localize("notifications_action_reply"),
                input_button_title: crate::i18n::localize("notifications_action_send"),
                input_placeholder: crate::i18n::localize("notifications_reply_placeholder"),
            }],
        },
    ]
}
