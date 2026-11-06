use std::collections::HashMap;

/// Pane choices retained separately for each route section.
#[derive(Clone)]
pub(crate) struct PaneVisibility {
    section: String,
    saved: HashMap<String, (bool, bool)>,
    pub sidebar_visible: bool,
    pub inspector_visible: bool,
}

impl PaneVisibility {
    pub fn new(path: &str) -> Self {
        let section = path.split('/').nth(1).unwrap_or_default().to_string();
        Self {
            sidebar_visible: !matches!(section.as_str(), "" | "explore"),
            inspector_visible: false,
            section,
            saved: HashMap::new(),
        }
    }

    pub fn navigate(&mut self, path: &str) {
        let section = path.split('/').nth(1).unwrap_or_default();
        if section == self.section {
            return;
        }
        self.saved.insert(
            self.section.clone(),
            (self.sidebar_visible, self.inspector_visible),
        );
        let (sidebar, inspector) = self
            .saved
            .get(section)
            .copied()
            .unwrap_or((!matches!(section, "" | "explore"), false));
        self.section = section.to_string();
        self.sidebar_visible = sidebar;
        self.inspector_visible = inspector;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_apply_to_restored_routes_and_keep_all_inspectors_closed() {
        for path in ["/", "/explore", "/projects/licenses", "/ui", "/tabs/new/3"] {
            let state = PaneVisibility::new(path);
            assert!(!state.inspector_visible, "{path}");
            assert_eq!(state.sidebar_visible, !matches!(path, "/" | "/explore"));
        }
    }

    #[test]
    fn explore_starts_closed_and_retains_deliberate_choices() {
        let mut state = PaneVisibility::new("/projects");
        state.inspector_visible = true;
        state.navigate("/explore");
        assert!(!state.sidebar_visible && !state.inspector_visible);
        state.sidebar_visible = true;
        state.navigate("/explore/detail/text/one");
        assert!(state.sidebar_visible);
        state.navigate("/projects/licenses");
        assert!(state.inspector_visible);
        state.navigate("/explore");
        assert!(state.sidebar_visible && !state.inspector_visible);
    }
}
