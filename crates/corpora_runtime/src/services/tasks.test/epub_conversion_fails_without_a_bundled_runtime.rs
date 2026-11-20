#[allow(unused_imports)]
use super::*;

// Test binaries sit outside an app bundle, so discovery finds no runtime.
#[gpui::test]
fn epub_conversion_fails_without_a_bundled_runtime(cx: &mut gpui::TestAppContext) {
    rust_i18n::set_locale("en");
    let directory = tempfile::tempdir().expect("create fixture directory");
    let input = directory.path().join("field-guide.epub");

    cx.update(|cx| start_epub_conversion(input, cx));
    let started = cx.read(|cx| cx.global::<TaskRegistry>().tasks[0].clone());
    assert_eq!(started.status, TaskStatus::Running);
    assert_eq!(started.label, "Convert field-guide.epub");

    cx.run_until_parked();

    let finished = cx.read(|cx| cx.global::<TaskRegistry>().tasks[0].clone());
    assert_eq!(finished.status, TaskStatus::Failed);
    assert_eq!(
        finished.error.as_deref(),
        Some("EPUB conversion is not available in this build.")
    );
    assert!(finished.finished_at.is_some());
    assert!(!directory.path().join("field-guide-tf").exists());
}
