#[allow(unused_imports)]
use super::*;

use corpora_conversion::ProgressPhase;

use crate::i18n::{localize, localize_with_args};

pub(crate) enum EpubConversionNotification<'a> {
    Running {
        name: &'a str,
        phase: Option<ProgressPhase>,
    },
    Succeeded {
        output: &'a str,
    },
    Failed {
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
    /// Keyed per task so each phase replaces the previous toast instead of stacking.
    pub(crate) fn push(self, id: TaskId, cx: &mut App) {
        crate::ui::toasts::replace(format!("epub-conversion-{id}"), self.toast(), cx);
    }

    fn toast(self) -> Toast {
        match self {
            Self::Running { name, phase } => {
                let key = match phase {
                    None => "tasks_epub_phase_starting",
                    Some(ProgressPhase::Validating) => "tasks_epub_phase_validating",
                    Some(ProgressPhase::Converting) => "tasks_epub_phase_converting",
                    Some(ProgressPhase::Finalizing) => "tasks_epub_phase_finalizing",
                };
                Toast::new(localize("tasks_epub_running_title"))
                    .body(localize_with_args(key, &[("name", name)]))
                    .severity(Severity::Info)
                    .stay(None)
            }
            Self::Succeeded { output } => Toast::new(localize("tasks_epub_succeeded_title"))
                .body(localize_with_args(
                    "tasks_epub_succeeded_body",
                    &[("output", output)],
                ))
                .severity(Severity::Success),
            Self::Failed {
                message,
                issues,
                more_issues,
                staging,
            } => {
                let mut body = match staging {
                    Some(staging) => localize_with_args(
                        "tasks_epub_cleanup_body",
                        &[("message", message), ("staging", staging)],
                    ),
                    None => message.to_string(),
                };
                for issue in issues {
                    body.push_str("\n• ");
                    body.push_str(issue);
                }
                if more_issues > 0 {
                    body.push('\n');
                    body.push_str(&localize_with_args(
                        "tasks_epub_more_issues",
                        &[("count", &more_issues.to_string())],
                    ));
                }
                Toast::new(localize("tasks_epub_failed_title"))
                    .body(body)
                    .severity(Severity::Danger)
                    .stay(None)
            }
            Self::Cancelled => Toast::new(localize("tasks_epub_cancelled_title"))
                .body(localize("tasks_epub_cancelled_body"))
                .severity(Severity::Warning)
                .stay(None),
            Self::Unknown { staging } => {
                let body = match staging {
                    Some(staging) => {
                        localize_with_args("tasks_epub_unknown_body", &[("staging", staging)])
                    }
                    None => localize("tasks_epub_unknown_no_staging_body"),
                };
                Toast::new(localize("tasks_epub_unknown_title"))
                    .body(body)
                    .severity(Severity::Danger)
                    .stay(None)
            }
        }
    }
}
