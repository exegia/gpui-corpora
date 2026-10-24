use crate::navigation;

pub(super) const EMPTY_TAB_PREFIX: &str = "/tabs/new/";

#[derive(Clone, Debug)]
pub(crate) struct WorkspaceTab {
    pub id: String,
    pub number: u64,
    pub path: String,
}

#[derive(Default)]
pub(crate) struct WorkspaceTabs {
    entries: Vec<WorkspaceTab>,
    active: Option<String>,
    next_empty_id: u64,
}

impl WorkspaceTabs {
    pub(crate) fn entries(&self) -> &[WorkspaceTab] {
        &self.entries
    }

    pub(crate) fn active_id(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub(crate) fn navigated(&mut self, path: &str) {
        self.active = self
            .entries
            .iter()
            .find(|tab| tab.path == path)
            .map(|tab| tab.id.clone());
    }

    pub(crate) fn restore_empty(&mut self, path: &str) {
        if let Some(number) = path
            .strip_prefix(EMPTY_TAB_PREFIX)
            .and_then(|id| id.parse().ok())
        {
            self.next_empty_id = number;
            self.entries.push(WorkspaceTab {
                id: path.to_string(),
                number,
                path: path.to_string(),
            });
            self.active = Some(path.to_string());
        }
    }

    pub(crate) fn new_empty(&mut self) -> String {
        self.next_empty_id += 1;
        let id = format!("{EMPTY_TAB_PREFIX}{}", self.next_empty_id);
        self.entries.push(WorkspaceTab {
            id: id.clone(),
            number: self.next_empty_id,
            path: id.clone(),
        });
        self.active = Some(id.clone());
        id
    }

    pub(crate) fn select(&mut self, id: &str) -> Option<String> {
        let tab = self.entries.iter().find(|tab| tab.id == id)?;
        self.active = Some(tab.id.clone());
        Some(tab.path.clone())
    }

    pub(crate) fn close(&mut self, id: &str) -> Option<String> {
        let index = self.entries.iter().position(|tab| tab.id == id)?;
        self.entries.remove(index);
        if self.active_id() != Some(id) {
            return None;
        }
        let next = self
            .entries
            .get(index.min(self.entries.len().saturating_sub(1)));
        self.active = next.map(|tab| tab.id.clone());
        Some(
            next.map(|tab| tab.path.clone())
                .unwrap_or_else(|| navigation::HOME.to_string()),
        )
    }
}

#[cfg(test)]
#[path = "workspace_tabs.test.rs"]
mod tests;
