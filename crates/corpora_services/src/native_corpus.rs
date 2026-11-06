//! Local corpus access through Context-Fabric. Call these operations on a worker.

use context_fabric_core::{
    Fabric, FeatureValue, Interrupt, MappedCompiledCorpus, MappedSearch, MappedSections,
    SectionOptions, TextOptions, TextRenderer,
};
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

pub const PAGE_SIZE: usize = 50;
pub const SEARCH_LIMIT: usize = 100;
const PASSAGE_CHAR_LIMIT: usize = 16_000;

#[derive(Clone, Default)]
pub struct Cancellation(Arc<AtomicBool>);

impl Cancellation {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    fn check(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.interrupted(), "Operation cancelled");
        Ok(())
    }
}

impl Interrupt for Cancellation {
    fn interrupted(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Owns one request generation; dropping it interrupts outstanding search work.
#[derive(Default)]
pub struct Requests {
    generation: u64,
    cancellation: Cancellation,
}

impl Requests {
    pub fn begin(&mut self) -> (u64, Cancellation) {
        self.cancel();
        self.cancellation = Cancellation::default();
        (self.generation, self.cancellation.clone())
    }
    pub fn cancel(&mut self) {
        self.cancellation.cancel();
        self.generation = self.generation.wrapping_add(1);
    }
    pub fn accepts(&self, generation: u64) -> bool {
        self.generation == generation
    }
}

impl Drop for Requests {
    fn drop(&mut self) {
        self.cancel();
    }
}

pub struct NativeCorpus {
    mapped: Arc<MappedCompiledCorpus>,
    renderer: TextRenderer,
    pub name: String,
    pub summary: String,
    pub types: Vec<String>,
    pub default_type: String,
}

#[derive(Clone, Debug)]
pub struct NodeLabel {
    pub node: u32,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct NodePage {
    pub kind: String,
    pub offset: usize,
    pub total: usize,
    pub nodes: Vec<NodeLabel>,
}

#[derive(Clone, Debug)]
pub struct Passage {
    pub node: u32,
    pub label: String,
    pub text: String,
    pub previous: Option<u32>,
    pub next: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct SearchResults {
    pub hits: Vec<NodeLabel>,
    pub capped: bool,
}

impl NativeCorpus {
    /// Opens a TF feature directory or a compiled CFR file using the native engine.
    pub fn open(path: &Path, cancel: &Cancellation) -> anyhow::Result<Arc<Self>> {
        cancel.check()?;
        let mapped = Arc::new(if path.is_dir() {
            Fabric::new(path).load_all()?
        } else {
            MappedCompiledCorpus::open(path)?
        });
        cancel.check()?;
        let sections = MappedSections::new(&mapped)?;
        let types: Vec<_> = mapped.otype_rank()?.into_keys().collect();
        let default_type = sections
            .section_types()
            .last()
            .cloned()
            .unwrap_or(mapped.slot_type()?);
        let summary = format!(
            "{} nodes · {} slots · {} node features · sections: {}",
            mapped.max_node(),
            mapped.max_slot()?,
            mapped.all_node_features(false).len(),
            sections.section_types().join(" / ")
        );
        let renderer = TextRenderer::new(&mapped)?;
        cancel.check()?;
        Ok(Arc::new(Self {
            mapped,
            renderer,
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            summary,
            types,
            default_type,
        }))
    }

    fn label(&self, node: u32) -> anyhow::Result<NodeLabel> {
        let sections = MappedSections::new(&self.mapped)?;
        let reference = sections
            .section_from_node(node, &SectionOptions::default())?
            .into_iter()
            .flatten()
            .map(|value| match value {
                FeatureValue::Str(s) => s.to_string(),
                FeatureValue::Int(i) => i.to_string(),
            })
            .collect::<Vec<_>>()
            .join(" / ");
        let kind = sections.node_type(node)?.unwrap_or("node");
        Ok(NodeLabel {
            node,
            label: if reference.is_empty() {
                format!("{kind} #{node}")
            } else {
                format!("{reference} · {kind} #{node}")
            },
        })
    }

    pub fn nodes(
        &self,
        kind: &str,
        offset: usize,
        cancel: &Cancellation,
    ) -> anyhow::Result<NodePage> {
        cancel.check()?;
        let nodes = self
            .mapped
            .typed_nodes(kind)?
            .ok_or_else(|| anyhow::anyhow!("Unknown node type: {kind}"))?;
        let offset = offset.min(nodes.len().saturating_sub(1)) / PAGE_SIZE * PAGE_SIZE;
        let labels = nodes
            .iter()
            .skip(offset)
            .take(PAGE_SIZE)
            .map(|&node| {
                cancel.check()?;
                self.label(node)
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(NodePage {
            kind: kind.to_owned(),
            offset,
            total: nodes.len(),
            nodes: labels,
        })
    }

    pub fn passage(&self, node: u32, cancel: &Cancellation) -> anyhow::Result<Passage> {
        cancel.check()?;
        anyhow::ensure!(
            node > 0 && node <= self.mapped.max_node(),
            "Invalid passage node: {node}"
        );
        let sections = MappedSections::new(&self.mapped)?;
        let kind = sections.node_type(node)?;
        let mut text = self.renderer.text(node, &TextOptions::default())?;
        if let Some((end, _)) = text.char_indices().nth(PASSAGE_CHAR_LIMIT) {
            text.truncate(end);
            text.push_str(
                "\n[Passage preview truncated. Select a smaller section to read further.]",
            );
        }
        cancel.check()?;
        Ok(Passage {
            node,
            label: self.label(node)?.label,
            text,
            previous: sections.previous(node, kind)?.first().copied(),
            next: sections.next(node, kind)?.first().copied(),
        })
    }

    pub fn search(&self, template: &str, cancel: &Cancellation) -> anyhow::Result<SearchResults> {
        cancel.check()?;
        anyhow::ensure!(
            !template.trim().is_empty(),
            "Enter a Context-Fabric search template"
        );
        let rows = MappedSearch::new(&self.mapped)?
            .with_interrupt(cancel)
            .search_first_nodes(template, Some(SEARCH_LIMIT + 1))?;
        cancel.check()?;
        let capped = rows.len() > SEARCH_LIMIT;
        let sections = MappedSections::new(&self.mapped)?;
        let hits = rows
            .into_iter()
            .take(SEARCH_LIMIT)
            .map(|node| {
                cancel.check()?;
                let passage = if sections.node_type(node)? == Some(self.default_type.as_str()) {
                    node
                } else {
                    sections
                        .up(node, Some(&self.default_type))?
                        .first()
                        .copied()
                        .unwrap_or(node)
                };
                let mut label = self.label(passage)?;
                label.label = format!("Match #{node} · {}", label.label);
                Ok(label)
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(SearchResults { hits, capped })
    }
}

#[cfg(test)]
#[path = "native_corpus.test.rs"]
mod tests;
