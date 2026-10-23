use gpui_kit::component::setting::{SettingGroup, SettingItem, SettingPage};

use super::*;

impl SettingsPage {
    pub(super) fn page(&self) -> SettingPage {
        let (key, title, _) = SECTIONS[self.selected];
        let page = SettingPage::new(crate::i18n::localize(title)).resettable(false);
        if key == "general" {
            return page.group(general::section());
        }
        let snapshot = self.notifications.clone();
        let workspace = self.workspace.clone();
        let event_log = self.event_log.clone();
        page.group(
            SettingGroup::new().item(SettingItem::render(move |_, _, cx| match key {
                "notifications" => notifications_section::section(&snapshot).into_any_element(),
                "workspace" => workspace::section(workspace.clone(), cx).into_any_element(),
                "shortcuts" => general::shortcuts_section(cx).into_any_element(),
                "storage" => general::storage_section().into_any_element(),
                "developer" => developer::section(&event_log, cx).into_any_element(),
                _ => div().into_any_element(),
            })),
        )
    }
}
