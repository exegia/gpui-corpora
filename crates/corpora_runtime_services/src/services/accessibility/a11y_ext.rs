#[allow(unused_imports)]
use super::*;

/// App-level accessibility conveniences layered on gpui's element builders.
pub trait A11yExt: StatefulInteractiveElement {
    /// Set the accessible role and label together — the minimum for the
    /// element to be announced as one meaningful node. Requires `.id(...)`
    /// on the same element.
    fn a11y(self, role: Role, label: impl Into<SharedString>) -> Self {
        self.role(role).aria_label(label)
    }

    /// Report the node as a live region: assistive technology announces
    /// content changes at `politeness`. Only takes effect on elements that
    /// have an id and a role; gpui exposes live-region state only through
    /// its synthetic-children hook, which this wraps.
    fn a11y_live(self, politeness: Live) -> Self {
        self.a11y_synthetic_children(move |builder| {
            builder.parent_node().set_live(politeness);
        })
    }
}
