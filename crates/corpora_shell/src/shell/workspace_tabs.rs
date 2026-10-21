use crate::navigation;

pub(super) const EMPTY_TAB_PREFIX: &str = "/tabs/new/";

#[derive(Clone, Debug)]
pub(crate) struct WorkspaceTab<T> {
    pub id: String,
    pub number: u64,
    pub path: String,
    pub state: T,
}

pub(crate) struct WorkspaceTabs<T> {
    entries: Vec<WorkspaceTab<T>>,
    active: Option<String>,
    next_empty_id: u64,
}

impl<T> Default for WorkspaceTabs<T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            active: None,
            next_empty_id: 0,
        }
    }
}

impl<T> WorkspaceTabs<T> {
    pub(crate) fn entries(&self) -> &[WorkspaceTab<T>] {
        &self.entries
    }

    pub(crate) fn active_id(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub(crate) fn active(&self) -> Option<&WorkspaceTab<T>> {
        self.entries
            .iter()
            .find(|tab| Some(tab.id.as_str()) == self.active_id())
    }

    pub(crate) fn active_mut(&mut self) -> Option<&mut WorkspaceTab<T>> {
        let active = self.active.as_deref();
        self.entries
            .iter_mut()
            .find(|tab| Some(tab.id.as_str()) == active)
    }

    pub(crate) fn navigated(&mut self, path: &str) {
        if let Some(tab) = self.active_mut() {
            tab.path = path.to_string();
        }
    }

    pub(crate) fn restore(&mut self, path: &str, state: T) {
        self.next_empty_id = path
            .strip_prefix(EMPTY_TAB_PREFIX)
            .and_then(|id| id.parse::<u64>().ok())
            .unwrap_or(1)
            .saturating_sub(1);
        self.new_empty(state);
        self.navigated(path);
    }

    pub(crate) fn new_empty(&mut self, state: T) -> String {
        self.next_empty_id += 1;
        let id = format!("{EMPTY_TAB_PREFIX}{}", self.next_empty_id);
        self.entries.push(WorkspaceTab {
            id: id.clone(),
            number: self.next_empty_id,
            path: id.clone(),
            state,
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
