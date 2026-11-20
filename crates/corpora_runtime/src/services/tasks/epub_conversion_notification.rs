#[allow(unused_imports)]
use super::*;

use std::rc::Rc;

use corpora_conversion::ProgressPhase;
use gpui::{
    InteractiveElement, ParentElement, SharedString, Styled, div, prelude::FluentBuilder, px,
};
use gpui_kit::{
    base::v_flex,
    component::{
        WindowExt,
        button::{Button, ButtonVariants},
        dialog::Dialog,
        notification::Notification,
        scroll::ScrollableElement,
    },
};

use crate::i18n::{localize, localize_with_args};

const DETAILS_WIDTH: f32 = 640.;
const DETAILS_MAX_HEIGHT: f32 = 420.;

/// Notification ID type; the task ID is the key, so each update replaces the last.
struct EpubConversionNote;

pub(crate) enum EpubConversionNotification<'a> {
    Running {
        name: &'a str,
        phase: Option<ProgressPhase>,
    },
    Succeeded {
        output: &'a str,
    },
    Failed {
        name: &'a str,
        message: &'a str,
        issues: &'a [String],
        more_issues: usize,
        staging: Option<&'a str>,
    },
    Cancelled,
    Unknown {
        staging: Option<&'a str>,
    },
}

impl EpubConversionNotification<'_> {
    /// Show the notification in the active window, or the first window when none is active.
    pub(crate) fn push(self, id: TaskId, cx: &mut App) {
        let Some(window) = cx
            .active_window()
            .or_else(|| cx.windows().into_iter().next())
        else {
            tracing::debug!(target: "corpora::tasks", "no window for EPUB notification");
            return;
        };
        let note = self
            .notification()
            .id1::<EpubConversionNote>(id.to_string());
        let _ = window.update(cx, |_, window, cx| window.push_notification(note, cx));
    }

    fn notification(self) -> Notification {
        match self {
            Self::Running { name, phase } => {
                let key = match phase {
                    None => "tasks_epub_phase_starting",
                    Some(ProgressPhase::Validating) => "tasks_epub_phase_validating",
                    Some(ProgressPhase::Converting) => "tasks_epub_phase_converting",
                    Some(ProgressPhase::Finalizing) => "tasks_epub_phase_finalizing",
                };
                Notification::info(localize_with_args(key, &[("name", name)]))
                    .title(localize("tasks_epub_running_title"))
                    .autohide(false)
            }
            Self::Succeeded { output } => Notification::success(localize_with_args(
                "tasks_epub_succeeded_body",
                &[("output", output)],
            ))
            .title(localize("tasks_epub_succeeded_title")),
            Self::Failed {
                name,
                message,
                issues,
                more_issues,
                staging,
            } => {
                let mut summary = match staging {
                    Some(staging) => localize_with_args(
                        "tasks_epub_cleanup_body",
                        &[("message", message), ("staging", staging)],
                    ),
                    None => message.to_string(),
                };
                if !issues.is_empty() {
                    let count = (issues.len() + more_issues).to_string();
                    summary.push(' ');
                    summary.push_str(&localize_with_args(
                        "tasks_epub_issue_summary",
                        &[("count", &count)],
                    ));
                }
                let note = Notification::error(summary)
                    .title(localize("tasks_epub_failed_title"))
                    .autohide(false);
                if issues.is_empty() {
                    return note;
                }
                let details = Rc::new(IssueDetails {
                    title: localize_with_args("tasks_epub_details_title", &[("name", name)]).into(),
                    issues: issues.iter().cloned().map(SharedString::from).collect(),
                    more_issues,
                });
                note.action(move |_, _, _| {
                    let details = details.clone();
                    Button::new("epub-issue-details")
                        .label(localize("tasks_epub_details_button"))
                        .primary()
                        .on_click(move |_, window, cx| {
                            let details = details.clone();
                            window.open_dialog(cx, move |dialog, _, _| details.dialog(dialog));
                        })
                })
            }
            Self::Cancelled => Notification::warning(localize("tasks_epub_cancelled_body"))
                .title(localize("tasks_epub_cancelled_title"))
                .autohide(false),
            Self::Unknown { staging } => {
                let body = match staging {
                    Some(staging) => {
                        localize_with_args("tasks_epub_unknown_body", &[("staging", staging)])
                    }
                    None => localize("tasks_epub_unknown_no_staging_body"),
                };
                Notification::error(body)
                    .title(localize("tasks_epub_unknown_title"))
                    .autohide(false)
            }
        }
    }
}

/// Every retained blocking issue, shown in full in a scrollable dialog.
struct IssueDetails {
    title: SharedString,
    issues: Vec<SharedString>,
    more_issues: usize,
}

impl IssueDetails {
    fn dialog(&self, dialog: Dialog) -> Dialog {
        let more = (self.more_issues > 0).then(|| {
            localize_with_args(
                "tasks_epub_more_issues",
                &[("count", &self.more_issues.to_string())],
            )
        });
        dialog.title(self.title.clone()).w(px(DETAILS_WIDTH)).child(
            v_flex()
                .id("epub-issue-list")
                .gap_2()
                .max_h(px(DETAILS_MAX_HEIGHT))
                .text_sm()
                .children(self.issues.iter().map(|issue| div().child(issue.clone())))
                .when_some(more, |list, more| list.child(div().child(more)))
                .overflow_y_scrollbar(),
        )
    }
}
