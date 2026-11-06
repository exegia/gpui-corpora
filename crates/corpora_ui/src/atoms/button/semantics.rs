use gpui::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Pixels, Role, SharedString, Window, accesskit,
};

/// Kit's unstyled button gates activation but does not expose disabled or busy metadata.
pub(super) struct ButtonSemantics<E> {
    inner: E,
    label: SharedString,
    role: Role,
    disabled: bool,
    busy: bool,
}

impl<E: Element> ButtonSemantics<E> {
    pub(super) fn new(
        inner: E,
        label: SharedString,
        role: Role,
        disabled: bool,
        busy: bool,
    ) -> Self {
        Self {
            inner,
            label,
            role,
            disabled,
            busy,
        }
    }
}

impl<E: Element> IntoElement for ButtonSemantics<E> {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl<E: Element> Element for ButtonSemantics<E> {
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = E::PrepaintState;
    fn id(&self) -> Option<ElementId> {
        self.inner.id()
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.inner.source_location()
    }
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.inner.request_layout(id, inspector, window, cx)
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.inner
            .prepaint(id, inspector, bounds, state, window, cx)
    }
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.inner
            .paint(id, inspector, bounds, state, prepaint, window, cx);
    }
    fn a11y_role(&self) -> Option<accesskit::Role> {
        Some(self.role)
    }
    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        self.inner.write_a11y_info(node);
        node.set_label(self.label.to_string());
        if self.disabled {
            node.set_disabled();
        }
        if self.busy {
            node.set_busy();
        }
    }
}
