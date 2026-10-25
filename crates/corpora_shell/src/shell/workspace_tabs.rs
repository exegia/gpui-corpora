pub(super) const EMPTY_TAB_PREFIX: &str = "/tabs/new/";
const ROUTE_VIEW: &str = "route";

#[derive(Clone, Debug)]
pub(crate) struct WorkspaceTab<T> {
    pub id: String,
    pub number: u64,
    pub path: String,
    pub closable: bool,
    pub state: T,
}

/// Workspace tabs, plus the one view all route pages share. The route view is
/// never listed in the tab bar.
pub(crate) struct WorkspaceTabs<T> {
    entries: Vec<WorkspaceTab<T>>,
    route: Option<WorkspaceTab<T>>,
    active: Option<String>,
    next_empty_id: u64,
}

impl<T> Default for WorkspaceTabs<T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            route: None,
            active: None,
            next_empty_id: 0,
        }
    }
}

fn tab_id(path: &str) -> Option<&str> {
    if path.starts_with(EMPTY_TAB_PREFIX) {
        Some(path)
    } else if cfg!(debug_assertions) && path.split('/').nth(1) == Some("ui") {
        Some("/ui")
    } else {
        None
    }
}

impl<T> WorkspaceTabs<T> {
    pub(crate) fn entries(&self) -> &[WorkspaceTab<T>] {
        &self.entries
    }

    pub(crate) fn active_id(&self) -> Option<&str> {
        self.active.as_deref()
    }

    /// The active tab, or the route view while a route page shows.
    pub(crate) fn active(&self) -> Option<&WorkspaceTab<T>> {
        let active = self.active.as_deref()?;
        self.route
            .iter()
            .chain(&self.entries)
            .find(|view| view.id == active)
    }

    pub(crate) fn active_mut(&mut self) -> Option<&mut WorkspaceTab<T>> {
        let active = self.active.as_deref()?;
        self.route
            .iter_mut()
            .chain(&mut self.entries)
            .find(|view| view.id == active)
    }

    /// Activates the view for `path`: its tab for a tab path, else the route view.
    /// Returns false when that view does not exist yet; create it with `open`.
    pub(crate) fn show(&mut self, path: &str) -> bool {
        if let Some(id) = tab_id(path) {
            let Some(tab) = self.entries.iter_mut().find(|tab| tab.id == id) else {
                return false;
            };
            tab.path = path.to_string();
            self.active = Some(tab.id.clone());
            return true;
        }
        let Some(route) = &mut self.route else {
            return false;
        };
        route.path = path.to_string();
        self.active = Some(route.id.clone());
        true
    }

    /// Creates the view `show` could not find for `path` and activates it.
    pub(crate) fn open(&mut self, path: &str, state: T) {
        if self.show(path) {
            return;
        }
        if tab_id(path).is_none() {
            self.route = Some(WorkspaceTab {
                id: ROUTE_VIEW.to_string(),
                number: 0,
                path: path.to_string(),
                closable: false,
                state,
            });
            self.active = Some(ROUTE_VIEW.to_string());
            return;
        }
        let number = path
            .strip_prefix(EMPTY_TAB_PREFIX)
            .and_then(|id| id.parse::<u64>().ok())
            .unwrap_or(self.next_empty_id + 1);
        self.next_empty_id = self.next_empty_id.max(number);
        self.push(number, state);
    }

    /// Records where the route view's router went; tabs keep their own path.
    pub(crate) fn navigated(&mut self, path: &str) {
        if let Some(route) = self
            .active_mut()
            .filter(|view| view.id == ROUTE_VIEW || tab_id(path) == Some(view.id.as_str()))
        {
            route.path = path.to_string();
        }
    }

    /// Adds a permanent developer tab without changing the current view.
    #[cfg(debug_assertions)]
    pub(crate) fn pin(&mut self, path: &str, state: T) {
        if self.entries.iter().any(|tab| tab.id == path) {
            return;
        }
        self.entries.push(WorkspaceTab {
            id: path.to_string(),
            number: 0,
            path: path.to_string(),
            closable: false,
            state,
        });
    }

    pub(crate) fn new_empty(&mut self, state: T) -> String {
        self.next_empty_id += 1;
        self.push(self.next_empty_id, state)
    }

    fn push(&mut self, number: u64, state: T) -> String {
        let id = format!("{EMPTY_TAB_PREFIX}{number}");
        self.entries.push(WorkspaceTab {
            id: id.clone(),
            number,
            path: id.clone(),
            closable: true,
            state,
        });
        self.active = Some(id.clone());
        id
    }

    pub(crate) fn path(&self, id: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|tab| tab.id == id)
            .map(|tab| tab.path.clone())
    }

    pub(crate) fn insert_empty(&mut self, state: T) -> String {
        let active = self.active.clone();
        let path = self.new_empty(state);
        self.active = active;
        path
    }

    pub(crate) fn path_after_close(&self, id: &str) -> Option<String> {
        let index = self
            .entries
            .iter()
            .position(|tab| tab.id == id && tab.closable)?;
        self.entries
            .get(index + 1)
            .or_else(|| {
                index
                    .checked_sub(1)
                    .and_then(|index| self.entries.get(index))
            })
            .or(self.route.as_ref())
            .map(|tab| tab.path.clone())
    }

    #[cfg(test)]
    pub(crate) fn select(&mut self, id: &str) -> Option<String> {
        let tab = self.entries.iter().find(|tab| tab.id == id)?;
        self.active = Some(tab.id.clone());
        Some(tab.path.clone())
    }

    /// Closes a tab. Closing the active one shows its neighbor, else the route
    /// view, and returns that path; with neither left the window is empty.
    pub(crate) fn close(&mut self, id: &str) -> Option<String> {
        let index = self.entries.iter().position(|tab| tab.id == id)?;
        if !self.entries[index].closable {
            return None;
        }
        self.entries.remove(index);
        if self.active_id() != Some(id) {
            return None;
        }
        let next = self
            .entries
            .get(index.min(self.entries.len().saturating_sub(1)))
            .or(self.route.as_ref());
        self.active = next.map(|view| view.id.clone());
        next.map(|view| view.path.clone())
    }
}

#[cfg(test)]
#[path = "workspace_tabs.test.rs"]
mod tests;
