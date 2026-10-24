#[allow(unused_imports)]
use super::*;

pub struct SettingsPage {
    pub(crate) workspace: WeakEntity<Workspace>,
    pub(crate) notifications: NotificationRuntimeSnapshot,
    /// (event id, rendered description); the id is the row's stable identity,
    /// which the description string alone cannot provide.
    pub(crate) event_log: Vec<(EventId, String)>,
    pub(crate) _subscriptions: Vec<Subscription>,
}

impl SettingsPage {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let _subscriptions = vec![
            cx.observe_global_in::<LocaleState>(window, |_, _, cx| cx.notify()),
            cx.observe_global_in::<NativeNotificationState>(window, |this, _, cx| {
                this.notifications = notifications::snapshot(cx);
                cx.notify();
            }),
            cx.observe_global_in::<crate::events::AppEventQueue>(window, |this, _, cx| {
                // Peek, never drain: the workspace owns the drain. The observer
                // fires per emit, so the id dedupe drops already-logged rows.
                let mut appended = false;
                if let Some(queue) = cx.try_global::<crate::events::AppEventQueue>() {
                    for event in &queue.0 {
                        if this.event_log.iter().any(|(id, _)| *id == event.id) {
                            continue;
                        }
                        this.event_log
                            .push((event.id, format!("{:?} ({})", event.kind, event.id)));
                        appended = true;
                    }
                }
                let excess = this.event_log.len().saturating_sub(EVENT_LOG_CAP);
                this.event_log.drain(0..excess);
                if appended || excess > 0 {
                    cx.notify();
                }
            }),
        ];

        Self {
            workspace,
            notifications: notifications::snapshot(cx),
            event_log: Vec::new(),
            _subscriptions,
        }
    }
}

impl Render for SettingsPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = use_params(cx)
            .get("section")
            .map(ToString::to_string)
            .filter(|key| SECTIONS.iter().any(|(section, ..)| section == key))
            .unwrap_or_else(|| SECTIONS[0].0.to_string());
        let page = match selected.as_str() {
            "notifications" => {
                notifications_section::section(&self.notifications).into_any_element()
            }
            "workspace" => workspace::section(self.workspace.clone(), cx).into_any_element(),
            "shortcuts" => general::shortcuts_section(cx).into_any_element(),
            "storage" => general::storage_section().into_any_element(),
            "developer" => developer::section(&self.event_log, cx).into_any_element(),
            _ => general::section(cx).into_any_element(),
        };
        let sections = SECTIONS
            .iter()
            .map(|(key, name, icon)| (*key, crate::i18n::localize(name), *icon));
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(Heading::h1(crate::i18n::localize("settings_title")))
            .child(
                SettingsLayout::new("settings", sections, selected, |key, window, cx| {
                    use_navigate(window, cx).push(format!("{}/{key}", crate::navigation::SETTINGS));
                })
                .page(page),
            )
    }
}
