use corpora_services::corpus_client::{self, Config, Corpus, Passage, Section, SharedSession};
use corpora_ui::ely::{
    buttons::Button,
    forms::{Input, TextInput},
    typography::{Heading, Paragraph},
};
use gpui::{
    BorrowAppContext as _, Context, Entity, Global, IntoElement, Render, Subscription, Window, div,
    prelude::*,
};
use uuid::Uuid;

#[derive(Default)]
pub struct CorpusSession {
    session: Option<SharedSession>,
    label: Option<String>,
    generation: u64,
    signing_in: bool,
    sign_in_abort: Option<tokio::task::AbortHandle>,
}
impl Global for CorpusSession {}
impl CorpusSession {
    pub(crate) fn is_signed_in(cx: &gpui::App) -> bool {
        cx.try_global::<Self>()
            .is_some_and(|auth| auth.session.is_some())
    }
}

pub struct CorpusPage {
    email: Entity<TextInput>,
    password: Entity<TextInput>,
    status: String,
    running: bool,
    loaded: bool,
    shared: bool,
    corpora: Vec<Corpus>,
    next_cursor: Option<Uuid>,
    selected: Option<Corpus>,
    selected_ref: Option<String>,
    sections: Vec<Section>,
    section_path: Vec<String>,
    next_section: Option<u64>,
    passages: Vec<Passage>,
    next_offset: Option<u64>,
    _subscriptions: Vec<Subscription>,
}

impl CorpusPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let email = cx.new(|cx| {
            TextInput::new(window, cx)
                .label("Email")
                .placeholder("you@example.com")
        });
        let password = cx.new(|cx| {
            TextInput::new(window, cx)
                .label("Password")
                .placeholder("Password")
                .masked()
        });
        let observe = cx.observe_global::<CorpusSession>(|this, cx| {
            if cx.global::<CorpusSession>().session.is_none()
                && !cx.global::<CorpusSession>().signing_in
            {
                this.running = false;
                this.loaded = false;
                this.corpora.clear();
                this.passages.clear();
                this.selected = None;
                this.selected_ref = None;
                this.sections.clear();
                this.section_path.clear();
                this.next_section = None;
                this.next_cursor = None;
                this.next_offset = None;
            }
            cx.notify();
        });
        Self {
            email,
            password,
            status: String::new(),
            running: false,
            loaded: false,
            shared: false,
            corpora: Vec::new(),
            next_cursor: None,
            selected: None,
            selected_ref: None,
            sections: Vec::new(),
            section_path: Vec::new(),
            next_section: None,
            passages: Vec::new(),
            next_offset: None,
            _subscriptions: vec![observe],
        }
    }

    fn sign_in(&mut self, provider: Option<&'static str>, cx: &mut Context<Self>) {
        if self.running || cx.global::<CorpusSession>().signing_in {
            return;
        }
        let config = match Config::from_environment() {
            Ok(value) => value,
            Err(error) => {
                self.status = error.to_string();
                cx.notify();
                return;
            }
        };
        let Some((runtime, _)) = crate::tokio_runtime::runtime_and_client(cx) else {
            self.status = "Network runtime is unavailable.".into();
            cx.notify();
            return;
        };
        let email = self.email.read(cx).text().trim().to_owned();
        let password = self.password.read(cx).text().to_owned();
        if provider.is_none() && (email.is_empty() || password.is_empty()) {
            self.status = "Enter your email and password.".into();
            cx.notify();
            return;
        }
        self.password.update(cx, |input, cx| input.set_text("", cx));
        self.running = true;
        self.status = if provider.is_some() {
            "Complete sign-in in your browser."
        } else {
            "Signing in…"
        }
        .into();
        let generation = cx.update_global::<CorpusSession, _>(|auth, _| {
            auth.signing_in = true;
            auth.generation += 1;
            auth.generation
        });
        let task = runtime.spawn(async move {
            match provider {
                Some(provider) => corpus_client::oauth_sign_in(&config, provider).await,
                None => corpus_client::password_sign_in(&config, email, password).await,
            }
        });
        cx.update_global::<CorpusSession, _>(|auth, _| {
            auth.sign_in_abort = Some(task.abort_handle())
        });
        cx.spawn(async move |view, cx| {
            let result = task.await;
            let status = cx.update(|cx| {
                if cx.global::<CorpusSession>().generation != generation {
                    return None;
                }
                let outcome = match result {
                    Ok(Ok(session)) => {
                        let label = session.email.clone().unwrap_or_else(|| "Signed in".into());
                        let shared = corpus_client::share(session);
                        cx.update_global::<CorpusSession, _>(|auth, _| {
                            auth.session = Some(shared);
                            auth.label = Some(label);
                            auth.signing_in = false;
                            auth.sign_in_abort = None;
                        });
                        crate::session::set_state(
                            crate::session::SessionState::SignedIn {
                                account_label: "Corpora".into(),
                            },
                            cx,
                        );
                        Ok(())
                    }
                    Ok(Err(error)) => Err(error.to_string()),
                    Err(_) => Err("Sign-in stopped unexpectedly.".into()),
                };
                if outcome.is_err() {
                    cx.update_global::<CorpusSession, _>(|auth, _| {
                        auth.signing_in = false;
                        auth.sign_in_abort = None;
                    });
                }
                Some(outcome)
            });
            view.update(cx, |this, cx| {
                this.running = false;
                match status {
                    Some(Ok(())) => {
                        this.status = "Signed in. Loading your corpora…".into();
                        this.load_corpora(false, false, cx);
                    }
                    Some(Err(error)) => this.status = error,
                    None => return,
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn cancel_sign_in(&mut self, cx: &mut Context<Self>) {
        cx.update_global::<CorpusSession, _>(|auth, _| {
            auth.generation += 1;
            auth.signing_in = false;
            if let Some(task) = auth.sign_in_abort.take() {
                task.abort();
            }
        });
        self.running = false;
        self.status = "Sign-in cancelled.".into();
        cx.notify();
    }

    fn sign_out(&mut self, cx: &mut Context<Self>) {
        let (session, generation) = cx.update_global::<CorpusSession, _>(|auth, _| {
            auth.generation += 1;
            auth.label = None;
            auth.signing_in = false;
            (auth.session.take(), auth.generation)
        });
        crate::session::set_state(crate::session::SessionState::SignedOut, cx);
        self.corpora.clear();
        self.passages.clear();
        self.selected = None;
        self.running = false;
        self.status = "Signed out on this device.".into();
        if let Some(session) = session {
            let (Ok(config), Some((runtime, _))) = (
                Config::from_environment(),
                crate::tokio_runtime::runtime_and_client(cx),
            ) else {
                self.status = "Signed out locally; server sign-out could not be confirmed.".into();
                cx.notify();
                return;
            };
            let task =
                runtime.spawn(async move { corpus_client::sign_out(&config, session).await });
            cx.spawn(async move |view, cx| {
                let result = task.await;
                view.update(cx, |this, cx| {
                    if cx.global::<CorpusSession>().generation != generation {
                        return;
                    }
                    if !matches!(result, Ok(Ok(()))) {
                        this.status =
                            "Signed out locally; server sign-out could not be confirmed.".into();
                        cx.notify();
                    }
                })
                .ok();
            })
            .detach();
        }
        cx.notify();
    }

    fn load_corpora(&mut self, shared_mode: bool, append: bool, cx: &mut Context<Self>) {
        if self.running {
            return;
        }
        let auth = cx.global::<CorpusSession>();
        let Some(session) = auth.session.clone() else {
            return;
        };
        let generation = auth.generation;
        let Ok(config) = Config::from_environment() else {
            self.status = "Supabase configuration is unavailable.".into();
            cx.notify();
            return;
        };
        let Some((runtime, _)) = crate::tokio_runtime::runtime_and_client(cx) else {
            self.status = "Network runtime is unavailable.".into();
            cx.notify();
            return;
        };
        let cursor = if append { self.next_cursor } else { None };
        self.loaded = true;
        self.shared = shared_mode;
        self.selected = None;
        self.selected_ref = None;
        self.sections.clear();
        self.section_path.clear();
        self.next_section = None;
        self.passages.clear();
        if !append {
            self.corpora.clear();
            self.next_cursor = None;
        }
        self.running = true;
        self.status = "Loading corpora…".into();
        let task = runtime
            .spawn(async move { corpus_client::list(&config, session, shared_mode, cursor).await });
        cx.spawn(async move |view, cx| {
            let result = task.await;
            view.update(cx, |this, cx| {
                if cx.global::<CorpusSession>().generation != generation {
                    return;
                }
                this.running = false;
                match result {
                    Ok(Ok((items, next))) => {
                        this.corpora.extend(items);
                        this.next_cursor = next;
                        this.status = if this.corpora.is_empty() {
                            "No corpora found."
                        } else {
                            "Choose a ready corpus to read."
                        }
                        .into();
                    }
                    Ok(Err(error)) => this.status = error.to_string(),
                    Err(_) => this.status = "Corpus request stopped unexpectedly.".into(),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn load_sections(
        &mut self,
        corpus: Corpus,
        path: Vec<String>,
        append: bool,
        cx: &mut Context<Self>,
    ) {
        if self.running {
            return;
        }
        let offset = if append {
            self.next_section.unwrap_or(0)
        } else {
            0
        };
        let auth = cx.global::<CorpusSession>();
        let Some(session) = auth.session.clone() else {
            return;
        };
        let generation = auth.generation;
        let Ok(config) = Config::from_environment() else {
            self.status = "Supabase configuration is unavailable.".into();
            cx.notify();
            return;
        };
        let Some((runtime, _)) = crate::tokio_runtime::runtime_and_client(cx) else {
            self.status = "Network runtime is unavailable.".into();
            cx.notify();
            return;
        };
        self.running = true;
        self.status = "Loading sections…".into();
        let parent = path.last().cloned();
        let task = runtime.spawn(async move {
            corpus_client::sections(&config, session, corpus.id, offset, parent).await
        });
        cx.spawn(async move |view, cx| {
            let result = task.await;
            view.update(cx, |this, cx| {
                if cx.global::<CorpusSession>().generation != generation {
                    return;
                }
                this.running = false;
                match result {
                    Ok(Ok(page)) => {
                        if !append {
                            this.sections.clear();
                            this.section_path = path;
                        }
                        this.sections.extend(page.items);
                        this.next_section = page.next_offset;
                        this.status = "Sections loaded.".into();
                    }
                    Ok(Err(error)) => this.status = error.to_string(),
                    Err(_) => this.status = "Section request stopped unexpectedly.".into(),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn load_content(
        &mut self,
        corpus: Corpus,
        reference: Option<String>,
        append: bool,
        cx: &mut Context<Self>,
    ) {
        if self.running {
            return;
        }
        let auth = cx.global::<CorpusSession>();
        let Some(session) = auth.session.clone() else {
            return;
        };
        let generation = auth.generation;
        let Ok(config) = Config::from_environment() else {
            self.status = "Supabase configuration is unavailable.".into();
            cx.notify();
            return;
        };
        let Some((runtime, _)) = crate::tokio_runtime::runtime_and_client(cx) else {
            self.status = "Network runtime is unavailable.".into();
            cx.notify();
            return;
        };
        let offset = if append {
            self.next_offset.unwrap_or(0)
        } else {
            0
        };
        let new_corpus = self
            .selected
            .as_ref()
            .is_none_or(|current| current.id != corpus.id);
        if new_corpus {
            self.sections.clear();
            self.section_path.clear();
            self.next_section = None;
        }
        self.begin_content_load(corpus.clone(), reference.clone(), append);
        let task = runtime.spawn(async move {
            let sections = if new_corpus {
                Some(corpus_client::sections(&config, session.clone(), corpus.id, 0, None).await)
            } else {
                None
            };
            let content =
                corpus_client::content(&config, session, corpus.id, offset, reference).await;
            (sections, content)
        });
        cx.spawn(async move |view, cx| {
            let result = task.await;
            view.update(cx, |this, cx| {
                if cx.global::<CorpusSession>().generation != generation {
                    return;
                }
                this.running = false;
                match result {
                    Ok((sections, Ok(content))) => {
                        let sections_unavailable = matches!(&sections, Some(Err(_)));
                        if let Some(Ok(sections)) = sections {
                            this.sections = sections.items;
                            this.next_section = sections.next_offset;
                        }
                        this.passages.extend(content.passages);
                        this.next_offset = content.next_offset;
                        this.status = if sections_unavailable {
                            "Text loaded; sections could not be loaded."
                        } else if this.passages.is_empty() {
                            "No text found in this corpus."
                        } else {
                            "Corpus loaded."
                        }
                        .into();
                    }
                    Ok((_, Err(error))) => this.status = error.to_string(),
                    Err(_) => this.status = "Corpus request stopped unexpectedly.".into(),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn begin_content_load(&mut self, corpus: Corpus, reference: Option<String>, append: bool) {
        if !append {
            self.passages.clear();
            self.next_offset = None;
        }
        self.selected_ref = reference;
        self.selected = Some(corpus);
        self.running = true;
        self.status = "Loading corpus text…".into();
    }
}

impl Render for CorpusPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let auth = cx.global::<CorpusSession>();
        let signed_in = auth.session.is_some();
        let label = auth.label.clone().unwrap_or_default();
        let busy = self.running || auth.signing_in;
        let signing_in = auth.signing_in;
        if signed_in && !self.loaded && !self.running {
            self.loaded = true;
            let view = cx.entity().downgrade();
            cx.defer(move |cx| {
                view.update(cx, |this, cx| this.load_corpora(false, false, cx))
                    .ok();
            });
        }
        let mut root = div()
            .id("corpus-browser")
            .flex()
            .flex_col()
            .gap_4()
            .w_full()
            .child(Heading::h1(if signed_in {
                "Your corpora"
            } else {
                "Sign in to Corpora"
            }));
        root = root.child(super::nav_button(
            "local-corpus",
            "Open a local corpus",
            "/explore",
        ));
        if !signed_in {
            root = root
                .child(Paragraph::new(
                    "Sign in to open the corpora you own or have been invited to read.",
                ))
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_3()
                        .child(
                            Button::new("sign-in-apple", "Continue with Apple")
                                .disabled(busy)
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.sign_in(Some("apple"), cx)),
                                ),
                        )
                        .child(
                            Button::new("sign-in-google", "Continue with Google")
                                .disabled(busy)
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.sign_in(Some("google"), cx)),
                                ),
                        ),
                )
                .child(div().max_w(gpui::rems(24.0)).child(Input::new(&self.email)))
                .child(
                    div()
                        .max_w(gpui::rems(24.0))
                        .child(Input::new(&self.password)),
                )
                .child(
                    div().max_w(gpui::rems(24.0)).child(
                        Button::new("sign-in-email", "Sign in with email")
                            .disabled(busy)
                            .on_click(cx.listener(|this, _, _, cx| this.sign_in(None, cx))),
                    ),
                );
            if signing_in {
                root = root.child(
                    Button::new("cancel-sign-in", "Cancel sign-in")
                        .on_click(cx.listener(|this, _, _, cx| this.cancel_sign_in(cx))),
                );
            }
        } else {
            root =
                root.child(Paragraph::new(format!("Signed in as {label}")))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_3()
                            .child(
                                Button::new("my-corpora", "My corpora")
                                    .disabled(busy)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.load_corpora(false, false, cx)
                                    })),
                            )
                            .child(
                                Button::new("shared-corpora", "Shared with me")
                                    .disabled(busy)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.load_corpora(true, false, cx)
                                    })),
                            )
                            .child(
                                Button::new("sign-out", "Sign out")
                                    .on_click(cx.listener(|this, _, _, cx| this.sign_out(cx))),
                            ),
                    );
            if let Some(corpus) = self.selected.clone() {
                root = root.child(Heading::h2(corpus.title.clone()));
                if !self.section_path.is_empty() {
                    let corpus = corpus.clone();
                    let mut parent = self.section_path.clone();
                    parent.pop();
                    root = root.child(
                        Button::new("parent-sections", "Back to parent sections")
                            .disabled(busy)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.load_sections(corpus.clone(), parent.clone(), false, cx)
                            })),
                    );
                }
                if !self.sections.is_empty() {
                    root = root.child(Paragraph::new("Sections"));
                    root =
                        root.children(self.sections.iter().enumerate().map(|(index, section)| {
                            let corpus = corpus.clone();
                            let reference = section.reference.clone();
                            let mut path = self.section_path.clone();
                            path.push(reference.clone());
                            let child_corpus = corpus.clone();
                            let item =
                                Button::new(format!("corpus-section-{index}"), reference.clone())
                                    .disabled(busy)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.load_content(
                                            corpus.clone(),
                                            Some(reference.clone()),
                                            false,
                                            cx,
                                        )
                                    }));
                            let mut row = div().flex().gap_2().child(item);
                            if section.child_count > 0 {
                                row = row.child(
                                    Button::new(
                                        format!("child-sections-{index}"),
                                        "Browse subsections",
                                    )
                                    .disabled(busy)
                                    .on_click(cx.listener(
                                        move |this, _, _, cx| {
                                            this.load_sections(
                                                child_corpus.clone(),
                                                path.clone(),
                                                false,
                                                cx,
                                            )
                                        },
                                    )),
                                );
                            }
                            row
                        }));
                    if self.next_section.is_some() {
                        let corpus = corpus.clone();
                        root = root.child(
                            Button::new("more-sections", "More sections")
                                .disabled(busy)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.load_sections(
                                        corpus.clone(),
                                        this.section_path.clone(),
                                        true,
                                        cx,
                                    )
                                })),
                        );
                    }
                }
                if self.selected_ref.is_some() {
                    let corpus = corpus.clone();
                    root = root.child(
                        Button::new("all-corpus-text", "Show all text")
                            .disabled(busy)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.load_content(corpus.clone(), None, false, cx)
                            })),
                    );
                }
                root = root.children(self.passages.iter().map(|passage| {
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(Paragraph::new(passage.reference.clone()))
                        .child(Paragraph::new(passage.text.clone()))
                }));
                if self.next_offset.is_some() {
                    let reference = self.selected_ref.clone();
                    root = root.child(
                        Button::new("more-text", "More text")
                            .disabled(busy)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.load_content(corpus.clone(), reference.clone(), true, cx)
                            })),
                    );
                }
            } else {
                root = root.children(self.corpora.iter().cloned().map(|corpus| {
                    let title = format!(
                        "{} · {}{}",
                        corpus.title,
                        corpus.state,
                        corpus
                            .language
                            .as_ref()
                            .map(|lang| format!(" · {lang}"))
                            .unwrap_or_default()
                    );
                    Button::new(format!("open-corpus-{}", corpus.id), title)
                        .disabled(busy || corpus.state != "ready")
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.load_content(corpus.clone(), None, false, cx)
                        }))
                }));
                if self.next_cursor.is_some() {
                    let shared = self.shared;
                    root = root.child(
                        Button::new("more-corpora", "More corpora")
                            .disabled(busy)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.load_corpora(shared, true, cx)
                            })),
                    );
                }
            }
        }
        if !self.status.is_empty() {
            root = root.child(Paragraph::new(self.status.clone()));
        }
        root
    }
}

#[cfg(test)]
#[path = "corpus.test.rs"]
mod tests;
