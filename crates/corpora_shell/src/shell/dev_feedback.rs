use gpui::{AnyElement, App, AssetSource, IntoElement, Window};

/// Window-local development feedback; ordinary host rendering in release builds.
pub struct Devtools {
    #[cfg(debug_assertions)]
    feedback: Option<gpui_agentation::Agentation>,
}

impl Devtools {
    /// Compose development assets with the application's existing sources.
    pub fn application(app: gpui::Application, assets: impl AssetSource) -> gpui::Application {
        #[cfg(debug_assertions)]
        let assets = gpui_agentation::Assets::before(assets);
        app.with_assets(assets)
    }

    /// Initialize tooling in debug and the host UI in release builds.
    pub fn init(cx: &mut App) -> gpui::Result<()> {
        #[cfg(debug_assertions)]
        {
            gpui_agentation::init(cx)
        }
        #[cfg(not(debug_assertions))]
        {
            corpora_ui::ely::init(cx)
        }
    }

    pub(crate) fn mount(path: &str, settings: bool, window: &mut Window, cx: &mut App) -> Self {
        #[cfg(all(debug_assertions, not(test)))]
        {
            let feedback = Self::mount_feedback(path, settings, window, cx)
                .map_err(|error| tracing::error!(%error, "could not mount dev feedback"))
                .ok()
                .flatten();
            Self { feedback }
        }
        #[cfg(any(not(debug_assertions), test))]
        {
            let _ = (path, settings, window, cx);
            Self {
                #[cfg(debug_assertions)]
                feedback: None,
            }
        }
    }

    #[cfg(all(debug_assertions, not(test)))]
    fn mount_feedback(
        path: &str,
        settings: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Option<gpui_agentation::Agentation>, Box<dyn std::error::Error>> {
        if std::env::var("GPUI_AGENTATION").as_deref() == Ok("0") {
            return Ok(None);
        }
        let storage = std::env::var_os("GPUI_AGENTATION_STORE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../target/agentation/feedback.json")
            });
        let storage = if settings {
            std::env::var_os("GPUI_AGENTATION_SETTINGS_STORE")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| settings_storage(&storage))
        } else {
            storage
        };
        let (port_key, default_port) = if settings {
            ("GPUI_AGENTATION_SETTINGS_PORT", 4751)
        } else {
            ("GPUI_AGENTATION_PORT", 4748)
        };
        let port = std::env::var(port_key)
            .map(|value| value.parse())
            .unwrap_or(Ok(default_port))?;
        Ok(Some(gpui_agentation::Agentation::mount(
            "Corpora",
            &format!("gpui://corpora{path}"),
            storage,
            port,
            window,
            cx,
        )?))
    }

    #[track_caller]
    pub(crate) fn frame(
        &self,
        id: &str,
        label: &str,
        content: impl IntoElement,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        #[cfg(debug_assertions)]
        if let Some(feedback) = &self.feedback {
            return feedback.root(id, label, content, window, cx);
        }
        let _ = (id, label, window, cx);
        content.into_any_element()
    }

    pub(crate) fn set_route(&self, path: &str, window: &mut Window, cx: &mut App) {
        #[cfg(debug_assertions)]
        if let Some(feedback) = &self.feedback {
            feedback.set_route(&format!("gpui://corpora{path}"), window, cx);
        }
        #[cfg(not(debug_assertions))]
        let _ = (path, window, cx);
    }
}

#[cfg(debug_assertions)]
fn settings_storage(workspace: &std::path::Path) -> std::path::PathBuf {
    let mut name = workspace.file_stem().unwrap_or_default().to_os_string();
    name.push("-settings");
    let mut path = workspace.with_file_name(name);
    if let Some(extension) = workspace.extension() {
        path.set_extension(extension);
    }
    path
}

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

#[cfg(all(test, debug_assertions))]
#[path = "dev_feedback.test.rs"]
mod tests;
