use corpora_services::api_qa::{self, Report};
use corpora_ui::ely::{
    buttons::Button,
    typography::{Heading, Paragraph},
};
use gpui::{Context, IntoElement, Render, Window, div, prelude::*};

/// A retained native view for the disposable API verification workflow.
pub struct ApiQa {
    running: bool,
    report: Report,
}

impl ApiQa {
    pub fn new() -> Self {
        Self {
            running: false,
            report: Report::default(),
        }
    }

    fn run(&mut self, lifecycle: bool, cx: &mut Context<Self>) {
        if self.running {
            return;
        }
        let Some((runtime, _)) = corpora_runtime::tokio_runtime::runtime_and_client(cx) else {
            self.report.rows = vec!["The network runtime is unavailable.".into()];
            cx.notify();
            return;
        };
        self.running = true;
        self.report = Report::default();
        let task = runtime.spawn(async move {
            if lifecycle {
                api_qa::lifecycle().await
            } else {
                api_qa::public_checks().await
            }
        });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                this.running = false;
                this.report = match result {
                    Ok(report) => report,
                    Err(_) => Report { rows: vec!["QA task stopped unexpectedly. Inspect the saved receipt before retrying.".into()], ..Report::default() },
                };
                cx.notify();
            }).ok();
        }).detach();
        cx.notify();
    }
}

impl Render for ApiQa {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = if self.running {
            "Running requests. The lifecycle usually takes 1–3 minutes."
        } else if self.report.passed {
            "All requested checks passed."
        } else if self.report.rows.is_empty() {
            "Ready. Start with the public checks."
        } else {
            "A check needs attention. Read the result below before retrying."
        };
        div().id("api-qa-page").flex().flex_col().gap_4().w_full()
            .child(Heading::h1("API QA"))
            .child(Paragraph::new("Target: https://api.exegia.co/v1 · Real production requests"))
            .child(Paragraph::new("Public checks need no account. The lifecycle creates, publishes, and deletes one synthetic corpus, then signs out its dedicated test user."))
            .child(div().flex().flex_wrap().gap_3()
                .child(Button::new("qa-public", "Check public API")
                    .on_click(cx.listener(|this, _, _, cx| this.run(false, cx))))
                .child(Button::new("qa-lifecycle", "Run disposable corpus test")
                    .on_click(cx.listener(|this, _, _, cx| this.run(true, cx)))))
            .child(Paragraph::new(status))
            .child(Paragraph::new("Lifecycle setup: launch with CORPORA_QA_CREDENTIALS and a fresh CORPORA_QA_RECEIPT path. Only user sessions go into the app; never admin or Hub keys. See docs/api-qa.md."))
            .children(self.report.rows.iter().map(|row| div().text_sm().child(row.clone())))
    }
}
