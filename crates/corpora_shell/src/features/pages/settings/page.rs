use gpui_kit::component::setting::{SettingGroup, SettingItem, SettingPage};

use super::*;

impl SettingsPage {
    pub(super) fn page(&self) -> SettingPage {
        let (key, title, _) = SECTIONS[self.selected];
        let page = SettingPage::new(crate::i18n::localize(title))
            .resettable(false)
            .header_style(&StyleRefinement::default().hidden());
        let snapshot = self.notifications.clone();
        if key == "general" {
            return page
                .group(general::section())
                .group(
                    SettingGroup::new().item(SettingItem::render(move |_, _, _| {
                        notifications_section::section(&snapshot)
                    })),
                );
        }
        let workspace = self.workspace.clone();
        let event_log = self.event_log.clone();
        page.group(
            SettingGroup::new().item(SettingItem::render(move |_, _, cx| match key {
                "workspace" => workspace::section(workspace.clone(), cx).into_any_element(),
                "shortcuts" => general::shortcuts_section(cx).into_any_element(),
                "developer" => div()
                    .flex()
                    .flex_col()
                    .gap_8()
                    .child(general::storage_section())
                    .child(notifications_section::test_section())
                    .child(developer::section(&event_log, cx))
                    .into_any_element(),
                _ => div().into_any_element(),
            })),
        )
    }
}
