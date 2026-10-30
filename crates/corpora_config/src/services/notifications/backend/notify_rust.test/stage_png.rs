#[allow(unused_imports)]
use super::*;

pub(crate) fn stage_png(path: &Path) {
    fs::create_dir_all(path.parent().expect("png parent")).expect("icon dir");
    fs::write(path, b"stub png").expect("stub png");
}
