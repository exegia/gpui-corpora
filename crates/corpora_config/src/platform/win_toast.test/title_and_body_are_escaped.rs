#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn title_and_body_are_escaped() {
    let xml = toast_xml(&NotificationRequest::foreground(
        "A & <b> \"c\" 'd'",
        "5 > 4 & 3 < 4",
    ));
    assert!(xml.contains("<text id=\"1\">A &amp; &lt;b&gt; &quot;c&quot; &apos;d&apos;</text>"));
    assert!(xml.contains("<text id=\"3\">5 &gt; 4 &amp; 3 &lt; 4</text>"));
}
