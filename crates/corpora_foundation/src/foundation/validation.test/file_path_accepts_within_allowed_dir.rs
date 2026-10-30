#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn file_path_accepts_within_allowed_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let file_path = tmp.path().join("good.txt");
    fs::write(&file_path, "data").unwrap();

    let result = validate_file_path(file_path.to_str().unwrap(), &[tmp.path().to_path_buf()]);
    assert!(result.is_ok());
}
