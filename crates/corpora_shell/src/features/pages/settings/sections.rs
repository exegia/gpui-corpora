pub(crate) const SECTIONS: [(&str, &str, &[u8]); 4] = [
    (
        "general",
        "settings_section_general",
        include_bytes!("assets/tabs/general.svg"),
    ),
    (
        "workspace",
        "settings_section_workspace",
        include_bytes!("assets/tabs/workspace.svg"),
    ),
    (
        "shortcuts",
        "settings_shortcuts",
        include_bytes!("assets/tabs/shortcuts.svg"),
    ),
    (
        "developer",
        "settings_developer",
        include_bytes!("assets/tabs/developer.svg"),
    ),
];

pub(super) fn section_index(section: &str) -> usize {
    let section = match section {
        "notifications" => "general",
        "storage" | "dev" => "developer",
        section => section,
    };
    SECTIONS
        .iter()
        .position(|(key, ..)| *key == section)
        .unwrap_or(0)
}
