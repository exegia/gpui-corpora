#[allow(unused_imports)]
use super::*;

pub(crate) const SECTIONS: [(&str, &str, IconName); 6] = [
    (
        "general",
        "settings_section_general",
        IconName::SlidersHorizontal,
    ),
    (
        "notifications",
        "settings_section_notifications",
        IconName::Bell,
    ),
    (
        "workspace",
        "settings_section_workspace",
        IconName::LayoutDashboard,
    ),
    ("shortcuts", "settings_shortcuts", IconName::Keyboard),
    ("storage", "settings_storage", IconName::Database),
    ("developer", "settings_developer", IconName::Code),
];
