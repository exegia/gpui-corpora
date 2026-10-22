/// Which panes a page lays out around its content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Panes {
    pub sidebar: bool,
    pub inspector: bool,
    /// A second content pane beside the page, behind a draggable divider.
    pub split: bool,
    /// A bottom panel under the content panes, behind a draggable divider.
    pub dock: bool,
}

const PAGE: Panes = Panes {
    sidebar: true,
    inspector: true,
    split: false,
    dock: false,
};

/// The panes `path` offers. Workspace tabs get every pane; route pages are listed here.
pub fn panes(path: &str) -> Panes {
    match path.split('/').nth(1).unwrap_or_default() {
        "tabs" => Panes {
            split: true,
            dock: true,
            ..PAGE
        },
        _ => PAGE,
    }
}

#[cfg(test)]
#[path = "panes.test.rs"]
mod tests;
