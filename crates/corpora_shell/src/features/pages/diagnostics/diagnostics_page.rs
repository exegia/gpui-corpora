#[allow(unused_imports)]
use super::*;

pub struct DiagnosticsPage {
    pub(crate) _subscriptions: Vec<Subscription>,
}

impl DiagnosticsPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut subscriptions = Vec::new();
        subscriptions.push(
            cx.observe_global_in::<app_state::AppState>(window, |_, _, cx| {
                cx.notify();
            }),
        );
        subscriptions.push(cx.observe_global_in::<LifecycleState>(window, |_, _, cx| {
            cx.notify();
        }));
        subscriptions.push(
            cx.observe_global_in::<notifications::NativeNotificationState>(window, |_, _, cx| {
                cx.notify();
            }),
        );
        subscriptions.push(cx.observe_global_in::<capabilities::CapabilityRegistry>(
            window,
            |_, _, cx| {
                cx.notify();
            },
        ));
        subscriptions.push(
            cx.observe_global_in::<storage::StorageSnapshot>(window, |_, _, cx| {
                cx.notify();
            }),
        );
        subscriptions.push(cx.observe_global_in::<telemetry::TelemetrySnapshot>(
            window,
            |_, _, cx| {
                cx.notify();
            },
        ));
        subscriptions.push(
            cx.observe_global_in::<desktop_actions::DesktopActionsState>(window, |_, _, cx| {
                cx.notify();
            }),
        );
        subscriptions.push(
            cx.observe_global_in::<shortcuts::ShortcutState>(window, |_, _, cx| {
                cx.notify();
            }),
        );
        subscriptions.push(
            cx.observe_global_in::<undo_stack::UndoState>(window, |_, _, cx| {
                cx.notify();
            }),
        );
        subscriptions.push(
            cx.observe_global_in::<accessibility::AccessibilitySnapshot>(window, |_, _, cx| {
                cx.notify();
            }),
        );
        subscriptions.push(cx.observe_global_in::<error_surface::ErrorSurfaceState>(
            window,
            |_, _, cx| {
                cx.notify();
            },
        ));
        subscriptions.push(cx.observe_global_in::<crash_report::CrashReportSnapshot>(
            window,
            |_, _, cx| {
                cx.notify();
            },
        ));
        Self {
            _subscriptions: subscriptions,
        }
    }
}

impl Render for DiagnosticsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let outline = |id: &'static str, key: &str| {
            Button::new(id, crate::i18n::localize(key)).variant(ButtonVariant::Outline)
        };
        let actions = div()
            .flex()
            .flex_wrap()
            .gap_2()
            .child(
                outline("diagnostics-refresh", "diagnostics_refresh").on_click(|_, _, cx| {
                    crate::events::emit(crate::events::AppEventKind::DiagnosticsChanged, cx);
                }),
            )
            .child(
                outline("diagnostics-reset-first-run", "diagnostics_reset_first_run")
                    .on_click(|_, _, cx| crate::first_run::reset(cx)),
            )
            .child(
                outline("diagnostics-copy", "diagnostics_copy").on_click(|_, _, cx| {
                    let _ = crate::desktop_actions::copy_diagnostics(cx);
                }),
            )
            .child(
                outline("diagnostics-open-logs", "diagnostics_open_logs").on_click(|_, _, cx| {
                    let _ = crate::desktop_actions::open_logs_folder(cx);
                }),
            )
            .child(
                outline(
                    "diagnostics-dismiss-latest-error",
                    "diagnostics_dismiss_latest_error",
                )
                .on_click(|_, _, cx| {
                    if let Some(error) = crate::error_surface::latest(cx) {
                        crate::error_surface::dismiss(error.id, cx);
                    }
                }),
            )
            .child(
                outline(
                    "diagnostics-retry-crash-upload",
                    "diagnostics_retry_crash_upload",
                )
                .on_click(|_, _, cx| crate::crash_report::upload_pending_reports(cx)),
            )
            .when(cfg!(debug_assertions), |this| {
                this.child(
                    outline(
                        "diagnostics-trigger-test-panic",
                        "diagnostics_trigger_test_panic",
                    )
                    .variant(ButtonVariant::Danger)
                    .on_click(|_, window, cx| {
                        // Window-scoped: App::dispatch_action is broken on
                        // Windows (thread-local active-window lookup).
                        window.dispatch_action(Box::new(crate::app::TriggerTestPanic), cx);
                    }),
                )
            });
        let list = rows::build_diagnostic_rows(cx)
            .into_iter()
            .fold(DescriptionList::new().lined(), |list, (label, value)| {
                list.text(label, value)
            });

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(Heading::h1(crate::i18n::localize("diagnostics_title")))
            .child(actions)
            .child(list)
    }
}
