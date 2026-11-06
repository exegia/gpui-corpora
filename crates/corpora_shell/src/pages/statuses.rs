#[allow(unused_imports)]
use super::*;

pub fn statuses() -> impl IntoElement {
    page("Statuses", "The default project workflow.").children(
        Status::ALL
            .into_iter()
            .map(|status| Card::new().header(CardHeader::new(status.words()))),
    )
}
