#[allow(unused_imports)]
use super::*;

pub(crate) fn escaped(text: &str) -> std::borrow::Cow<'_, str> {
    quick_xml::escape::escape(text)
}
