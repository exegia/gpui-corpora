use gpui::{AnyElement, IntoElement};

#[track_caller]
pub(crate) fn target(id: &'static str, label: &str, child: impl IntoElement) -> AnyElement {
    #[cfg(debug_assertions)]
    {
        gpui_agentation::target(id, label, child).into_any_element()
    }
    #[cfg(not(debug_assertions))]
    {
        let _ = (id, label);
        child.into_any_element()
    }
}

#[cfg(debug_assertions)]
pub(super) fn mount(
    path: &str,
    window: &mut gpui::Window,
    cx: &mut gpui::App,
) -> Option<gpui::Entity<gpui_agentation::AnnotationOverlay>> {
    if std::env::var("GPUI_AGENTATION").as_deref() == Ok("0") {
        return None;
    }
    let storage = std::env::var_os("GPUI_AGENTATION_STORE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/agentation/feedback.json")
        });
    let port = match std::env::var("GPUI_AGENTATION_PORT") {
        Ok(value) => match value.parse() {
            Ok(port) => port,
            Err(error) => {
                tracing::error!(%error, "invalid GPUI_AGENTATION_PORT");
                return None;
            }
        },
        Err(_) => 4748,
    };
    match gpui_agentation::AnnotationOverlay::mount(
        "Corpora",
        &format!("gpui://corpora{path}"),
        storage,
        port,
        window,
        cx,
    ) {
        Ok(overlay) => Some(overlay),
        Err(error) => {
            tracing::error!(%error, "could not mount dev feedback");
            None
        }
    }
}
