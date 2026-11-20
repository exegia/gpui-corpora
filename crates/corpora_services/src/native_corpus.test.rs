use super::*;
use std::fs;

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native");
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().is_some_and(|ext| ext == "tf") {
            fs::copy(entry.path(), dir.path().join(entry.file_name())).unwrap();
        }
    }
    dir
}

#[test]
fn opens_tf_and_cfr_with_identical_passage_and_structure() {
    let dir = fixture();
    let cancel = Cancellation::default();
    let tf = NativeCorpus::open(dir.path(), &cancel).unwrap();
    assert_eq!(tf.default_type, "verse");
    let page = tf.nodes("verse", 0, &cancel).unwrap();
    assert_eq!(
        page.nodes.iter().map(|n| n.node).collect::<Vec<_>>(),
        [5, 6]
    );
    let first = tf.passage(5, &cancel).unwrap();
    assert!(first.label.contains("Genesis / 1"));
    assert_eq!(first.text, "hello world ");
    assert_eq!((first.previous, first.next), (None, Some(6)));
    let second = tf.passage(6, &cancel).unwrap();
    assert_eq!((second.previous, second.next), (Some(5), None));
    let cfr = dir.path().join("fixture.cfr");
    Fabric::new(dir.path()).compile(&cfr, "").unwrap();
    let mapped = NativeCorpus::open(&cfr, &cancel).unwrap();
    assert_eq!(mapped.passage(5, &cancel).unwrap().text, first.text);
}

#[test]
fn search_returns_containing_passage_and_reports_invalid_templates() {
    let dir = fixture();
    let cancel = Cancellation::default();
    let corpus = NativeCorpus::open(dir.path(), &cancel).unwrap();
    let found = corpus.search("word word=hello", &cancel).unwrap();
    assert_eq!(found.hits.iter().map(|h| h.node).collect::<Vec<_>>(), [5]);
    assert!(!found.capped);
    assert!(
        corpus
            .search("word word=absent", &cancel)
            .unwrap()
            .hits
            .is_empty()
    );
    assert!(corpus.search("unknown_type", &cancel).is_err());
    assert!(corpus.search("", &cancel).is_err());
    assert!(corpus.passage(0, &cancel).is_err());
    assert!(corpus.nodes("unknown_type", 0, &cancel).is_err());
}

#[test]
fn cancellation_and_supersession_reject_old_results() {
    let mut requests = Requests::default();
    let (first, first_cancel) = requests.begin();
    let (second, second_cancel) = requests.begin();
    assert!(first_cancel.interrupted());
    assert!(!requests.accepts(first));
    assert!(requests.accepts(second));
    requests.cancel();
    assert!(second_cancel.interrupted());
    assert!(!requests.accepts(second));
    let (_, last) = requests.begin();
    drop(requests);
    assert!(last.interrupted());
}

#[test]
fn cancelled_operations_and_bad_paths_return_errors() {
    let dir = fixture();
    let cancel = Cancellation::default();
    let corpus = NativeCorpus::open(dir.path(), &cancel).unwrap();
    cancel.cancel();
    assert!(corpus.search("word", &cancel).is_err());
    assert!(corpus.passage(5, &cancel).is_err());
    assert!(NativeCorpus::open(dir.path(), &cancel).is_err());
    assert!(NativeCorpus::open(&dir.path().join("missing.cfr"), &Cancellation::default()).is_err());
    fs::write(dir.path().join("broken.cfr"), "broken").unwrap();
    assert!(NativeCorpus::open(&dir.path().join("broken.cfr"), &Cancellation::default()).is_err());
}

#[test]
fn large_corpora_bound_search_pages_and_unicode_previews() {
    let dir = fixture();
    fs::write(
        dir.path().join("otype.tf"),
        "@node\n@valueType=str\n\n1-151\tword\n152\tverse\n153\tbook\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("oslots.tf"),
        "@edge\n\n152\t1-151\n153\t1-151\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("word.tf"),
        format!("@node\n@valueType=str\n\n1-151\t{}\n", "א".repeat(200)),
    )
    .unwrap();
    fs::write(
        dir.path().join("verse.tf"),
        "@node\n@valueType=int\n\n152\t1\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("book.tf"),
        "@node\n@valueType=str\n\n153\tGenesis\n",
    )
    .unwrap();
    let cancel = Cancellation::default();
    let corpus = NativeCorpus::open(dir.path(), &cancel).unwrap();
    let results = corpus.search("word", &cancel).unwrap();
    assert_eq!(results.hits.len(), SEARCH_LIMIT);
    assert!(results.capped);
    let page = corpus.nodes("word", 100, &cancel).unwrap();
    assert_eq!(
        (page.offset, page.total, page.nodes.len()),
        (100, 151, PAGE_SIZE)
    );
    let last = corpus.nodes("word", usize::MAX, &cancel).unwrap();
    assert_eq!(last.nodes[0].node, 151);
    let passage = corpus.passage(152, &cancel).unwrap();
    assert!(passage.text.contains("Passage preview truncated"));
    assert!(passage.text.chars().count() < PASSAGE_CHAR_LIMIT + 100);
}
