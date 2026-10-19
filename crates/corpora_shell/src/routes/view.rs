#[allow(unused_imports)]
use super::*;

/// The sole route tree, used in the workspace and in the optional dock's center.
pub fn view(workspace: WeakEntity<Workspace>) -> Routes {
    let frame = workspace.clone();
    let project = workspace.clone();
    let create_project = workspace.clone();
    let page = |select: fn(&shell::Pages) -> AnyElement| {
        let workspace = workspace.clone();
        move |_: &mut gpui::Window, cx: &mut App| match workspace.upgrade() {
            Some(entity) => select(&entity.read(cx).pages),
            None => pages::not_found().into_any_element(),
        }
    };
    Routes::new().child(
        Route::new()
            .path("/")
            .element(move |window, cx| shell::frame(frame.clone(), Outlet::new(), window, cx))
            .children([
                Route::new().index().element(|_, _| pages::home()),
                Route::new().path("explore").children([
                    Route::new().index().element(|_, _| pages::search()),
                    Route::new().path("detail/:type/:id").children([
                        Route::new()
                            .index()
                            .element(|_, cx| pages::explore_detail(cx)),
                        Route::new()
                            .path("viewer")
                            .element(|_, cx| pages::viewer(cx)),
                    ]),
                ]),
                Route::new().path("projects").children([
                    Route::new().index().element(|_, _| pages::projects()),
                    Route::new()
                        .path("detail/:projectId")
                        .element(move |_, cx| pages::project(project.clone(), false, cx)),
                    Route::new()
                        .path("create")
                        .element(move |_, cx| pages::project(create_project.clone(), true, cx)),
                    Route::new()
                        .path("statuses")
                        .children([Route::new().index().element(|_, _| pages::statuses())]),
                    Route::new().path("licenses").children([
                        Route::new().index().element(|_, _| pages::licenses()),
                        Route::new()
                            .path("detail/:licenseId")
                            .element(|_, cx| pages::license(cx)),
                        Route::new()
                            .path("create")
                            .element(|_, _| pages::license_create()),
                    ]),
                ]),
                Route::new().path("settings").children([
                    Route::new()
                        .index()
                        .element(page(|p| p.settings.clone().into_any_element())),
                    Route::new()
                        .path(":section")
                        .element(page(|p| p.settings.clone().into_any_element())),
                ]),
                Route::new()
                    .path("notifications")
                    .element(page(|p| p.notifications.clone().into_any_element())),
                Route::new()
                    .path("diagnostics")
                    .element(page(|p| p.diagnostics.clone().into_any_element())),
                Route::new()
                    .path("about")
                    .element(page(|p| p.about.clone().into_any_element())),
                Route::new().path("profile").children([
                    Route::new().index().element(|_, _| pages::information()),
                    Route::new()
                        .path("information")
                        .element(|_, _| pages::information()),
                    Route::new()
                        .path("authentication")
                        .element(|_, _| pages::authentication()),
                ]),
                Route::new().path("*").element(|_, _| pages::not_found()),
            ]),
    )
}
