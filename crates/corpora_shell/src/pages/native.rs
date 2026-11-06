use std::{path::PathBuf, sync::Arc};

use corpora_services::native_corpus::{
    Cancellation, NativeCorpus, NodePage, PAGE_SIZE, Passage, Requests, SearchResults,
};
use corpora_ui::ely::{
    buttons::{Button, ButtonVariant},
    forms::{Input, InputEvent, TextInput},
    typography::{Heading, Paragraph},
};
use gpui::{prelude::*, *};

enum Completion {
    Open(Arc<NativeCorpus>, NodePage, Option<Passage>),
    Nodes(NodePage),
    Passage(Passage),
    Search(SearchResults),
}

pub struct NativePage {
    _subscriptions: Vec<Subscription>,
    path: Entity<TextInput>,
    query: Entity<TextInput>,
    corpus: Option<Arc<NativeCorpus>>,
    nodes: Option<NodePage>,
    passage: Option<Passage>,
    results: Option<SearchResults>,
    requests: Requests,
    task: Option<Task<()>>,
    busy: Option<&'static str>,
    error: Option<String>,
}

impl NativePage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let path = cx.new(|cx| {
            TextInput::new(window, cx)
                .label("Local corpus path")
                .placeholder("TF feature directory or .cfr file")
        });
        let query = cx.new(|cx| {
            TextInput::new(window, cx)
                .label("Search template")
                .placeholder("word word=hello")
                .multi_line(2, 6)
        });
        let subscriptions = vec![
            cx.subscribe(&path, |this, _, event, cx| {
                if *event == InputEvent::Submit {
                    let path = PathBuf::from(this.path.read(cx).text().trim());
                    this.open(path, cx);
                }
            }),
            cx.subscribe(&query, |this, _, event, cx| {
                if *event == InputEvent::Submit {
                    this.search(cx);
                }
            }),
        ];
        Self {
            _subscriptions: subscriptions,
            path,
            query,
            corpus: None,
            nodes: None,
            passage: None,
            results: None,
            requests: Requests::default(),
            task: None,
            busy: None,
            error: None,
        }
    }

    fn run(
        &mut self,
        label: &'static str,
        work: impl FnOnce(Cancellation) -> anyhow::Result<Completion> + Send + 'static,
        cx: &mut Context<Self>,
    ) {
        let (generation, cancel) = self.requests.begin();
        self.busy = Some(label);
        self.error = None;
        let background = cx.background_spawn(async move { work(cancel) });
        self.task = Some(cx.spawn(async move |this, cx| {
            let result = background.await;
            let _ = this.update(cx, |this, cx| {
                this.complete(generation, result, cx);
            });
        }));
        cx.notify();
    }

    fn complete(
        &mut self,
        generation: u64,
        result: anyhow::Result<Completion>,
        cx: &mut Context<Self>,
    ) {
        if !self.requests.accepts(generation) {
            return;
        }
        self.busy = None;
        match result {
            Ok(Completion::Open(corpus, nodes, passage)) => {
                self.corpus = Some(corpus);
                self.nodes = Some(nodes);
                self.passage = passage;
            }
            Ok(Completion::Nodes(nodes)) => self.nodes = Some(nodes),
            Ok(Completion::Passage(passage)) => self.passage = Some(passage),
            Ok(Completion::Search(results)) => self.results = Some(results),
            Err(error) => self.error = Some(format!("{error:#}")),
        }
        cx.notify();
    }

    fn open(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.corpus = None;
        self.nodes = None;
        self.passage = None;
        self.results = None;
        self.run(
            "Opening corpus…",
            move |cancel| {
                let corpus = NativeCorpus::open(&path, &cancel)?;
                let nodes = corpus.nodes(&corpus.default_type, 0, &cancel)?;
                let passage = nodes
                    .nodes
                    .first()
                    .map(|node| corpus.passage(node.node, &cancel))
                    .transpose()?;
                Ok(Completion::Open(corpus, nodes, passage))
            },
            cx,
        );
    }

    fn choose(&mut self, cx: &mut Context<Self>) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: true,
            multiple: false,
            prompt: Some("Open corpus".into()),
        });
        cx.spawn(async move |this, cx| match receiver.await {
            Ok(Ok(Some(paths))) => {
                if let Some(path) = paths.into_iter().next() {
                    let _ = this.update(cx, |this, cx| this.open(path, cx));
                }
            }
            Ok(Ok(None)) => {}
            other => {
                let _ = this.update(cx, |this, cx| {
                    this.error = Some(format!("Could not choose a corpus: {other:?}"));
                    cx.notify();
                });
            }
        })
        .detach();
    }

    fn browse(&mut self, kind: String, offset: usize, cx: &mut Context<Self>) {
        let Some(corpus) = self.corpus.clone() else {
            return;
        };
        self.run(
            "Loading structure…",
            move |cancel| Ok(Completion::Nodes(corpus.nodes(&kind, offset, &cancel)?)),
            cx,
        );
    }

    fn read(&mut self, node: u32, cx: &mut Context<Self>) {
        let Some(corpus) = self.corpus.clone() else {
            return;
        };
        self.run(
            "Reading passage…",
            move |cancel| Ok(Completion::Passage(corpus.passage(node, &cancel)?)),
            cx,
        );
    }

    fn search(&mut self, cx: &mut Context<Self>) {
        let Some(corpus) = self.corpus.clone() else {
            return;
        };
        let template = self.query.read(cx).text().to_string();
        self.results = None;
        self.run(
            "Searching…",
            move |cancel| Ok(Completion::Search(corpus.search(&template, &cancel)?)),
            cx,
        );
    }
}

impl Render for NativePage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut content = div()
            .id("native-corpus")
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .gap_4()
            .child(Heading::h1("Local corpus"))
            .child(Paragraph::new(
                "Open a Text-Fabric feature directory or a compiled CFR file.",
            ))
            .child(Input::new(&self.path))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .child(
                        Button::new("native-open", "Open path").on_click(cx.listener(
                            |this, _, _, cx| {
                                let path = PathBuf::from(this.path.read(cx).text().trim());
                                this.open(path, cx);
                            },
                        )),
                    )
                    .child(
                        Button::new("native-choose", "Choose corpus…")
                            .on_click(cx.listener(|this, _, _, cx| this.choose(cx))),
                    ),
            );
        if let Some(busy) = self.busy {
            content = content.child(div().flex().gap_2().child(busy).child(
                Button::new("native-cancel", "Cancel").on_click(cx.listener(|this, _, _, cx| {
                    this.requests.cancel();
                    this.task = None;
                    this.busy = None;
                    cx.notify();
                })),
            ));
        }
        if let Some(error) = &self.error {
            content = content.child(Paragraph::new(error.clone()));
        }
        let Some(corpus) = &self.corpus else {
            return content;
        };
        content = content
            .child(Heading::h2(corpus.name.clone()))
            .child(Paragraph::new(corpus.summary.clone()));
        let mut types = div().flex().flex_wrap().gap_2();
        for (index, kind) in corpus.types.iter().enumerate() {
            let kind = kind.clone();
            types = types.child(
                Button::new(("native-type", index), kind.clone())
                    .variant(
                        if self.nodes.as_ref().is_some_and(|page| page.kind == kind) {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Outline
                        },
                    )
                    .on_click(cx.listener(move |this, _, _, cx| this.browse(kind.clone(), 0, cx))),
            );
        }
        content = content.child(Heading::h3("Structure")).child(types);
        if let Some(nodes) = &self.nodes {
            let kind = nodes.kind.clone();
            let offset = nodes.offset;
            let next_kind = kind.clone();
            let mut list = div()
                .id("native-structure-list")
                .max_h(rems(16.0))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .gap_1()
                .child(Paragraph::new(format!(
                    "{} · {}–{} of {}",
                    kind,
                    offset + usize::from(nodes.total > 0),
                    offset + nodes.nodes.len(),
                    nodes.total
                )));
            for entry in &nodes.nodes {
                let node = entry.node;
                list = list.child(
                    Button::new(("native-node", node as usize), entry.label.clone())
                        .variant(
                            if self
                                .passage
                                .as_ref()
                                .is_some_and(|passage| passage.node == node)
                            {
                                ButtonVariant::Primary
                            } else {
                                ButtonVariant::Ghost
                            },
                        )
                        .on_click(cx.listener(move |this, _, _, cx| this.read(node, cx))),
                );
            }
            content = content.child(list).child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("native-structure-prev", "Previous page")
                            .disabled(offset == 0)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.browse(kind.clone(), offset.saturating_sub(PAGE_SIZE), cx)
                            })),
                    )
                    .child(
                        Button::new("native-structure-next", "Next page")
                            .disabled(offset + PAGE_SIZE >= nodes.total)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.browse(next_kind.clone(), offset + PAGE_SIZE, cx)
                            })),
                    ),
            );
        }
        if let Some(passage) = &self.passage {
            let previous = passage.previous;
            let next = passage.next;
            content = content
                .child(Heading::h3(passage.label.clone()))
                .child(
                    div()
                        .flex()
                        .gap_2()
                        .child(
                            Button::new("native-passage-prev", "Previous passage")
                                .disabled(previous.is_none())
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    if let Some(node) = previous {
                                        this.read(node, cx);
                                    }
                                })),
                        )
                        .child(
                            Button::new("native-passage-next", "Next passage")
                                .disabled(next.is_none())
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    if let Some(node) = next {
                                        this.read(node, cx);
                                    }
                                })),
                        ),
                )
                .child(Paragraph::new(if passage.text.is_empty() {
                    "No text for this node in the corpus default format.".to_owned()
                } else {
                    passage.text.clone()
                }));
        }
        content = content.child(Heading::h3("Search"))
            .child(Paragraph::new("Enter a Context-Fabric template using the corpus node types and features. Up to 100 matches are shown."))
            .child(Input::new(&self.query))
            .child(Button::new("native-search", "Search corpus").on_click(cx.listener(|this, _, _, cx| this.search(cx))));
        if let Some(results) = &self.results {
            let mut hits = div()
                .id("native-search-results")
                .max_h(rems(20.0))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .gap_1();
            content = content.child(Paragraph::new(format!(
                "{} matches{}",
                results.hits.len(),
                if results.capped {
                    " shown; refine the query for more specific results"
                } else {
                    ""
                }
            )));
            for (index, hit) in results.hits.iter().enumerate() {
                let node = hit.node;
                hits = hits.child(
                    Button::new(("native-hit", index), hit.label.clone())
                        .on_click(cx.listener(move |this, _, _, cx| this.read(node, cx))),
                );
            }
            content = content.child(hits);
        }
        content
    }
}

#[cfg(test)]
#[path = "native.test.rs"]
mod tests;
