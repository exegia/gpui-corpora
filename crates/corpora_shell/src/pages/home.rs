#[allow(unused_imports)]
use super::*;

pub fn home() -> impl IntoElement {
    let metric = |id, label, value, note| {
        let card = Card::new()
            .flex_1()
            .flex_basis(gpui::rems(18.0))
            .min_w_0()
            .header(CardHeader::new(label))
            .child(Heading::h2(value))
            .child(Caption::new(note));
        crate::shell::dev_target(id, label, card)
    };
    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(Overline::new("Workspace overview"))
        .child(crate::shell::dev_target(
            "home-heading",
            "Workspace heading",
            Heading::h1("Your corpus workspace."),
        ))
        .child(Paragraph::new(
            "Explore your texts, organize projects, and make room for a closer reading.",
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_4()
                .child(metric(
                    "runtime-metric",
                    "Python runtime",
                    "Not connected",
                    "Connect a runtime to begin processing.",
                ))
                .child(metric(
                    "rpc-metric",
                    "RPC response",
                    "Not checked",
                    "No service connection configured.",
                ))
                .child(metric(
                    "access-metric",
                    "Access policy",
                    "Local workspace",
                    "Not signed in",
                )),
        )
        .child(crate::shell::dev_target(
            "runtime-connection",
            "Runtime connection card",
            Card::new()
                .header(
                    CardHeader::new("Runtime connection")
                        .description("Set up the services behind your workspace."),
                )
                .child(Paragraph::new("No runtime connected."))
                .child(div().flex().justify_end().child(nav_button(
                    "connection-settings",
                    "Connection settings",
                    "/settings",
                ))),
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_4()
                .child(crate::shell::dev_target(
                    "projects-card",
                    "Projects card",
                    Card::new()
                        .flex_1()
                        .flex_basis(gpui::rems(18.0))
                        .min_w_0()
                        .header(
                            CardHeader::new("Projects")
                                .description("A home for your document workflows."),
                        )
                        .child(div().flex().justify_end().child(nav_button(
                            "open-projects",
                            "Open projects",
                            "/projects",
                        ))),
                ))
                .child(crate::shell::dev_target(
                    "explore-card",
                    "Corpus explorer card",
                    Card::new()
                        .flex_1()
                        .flex_basis(gpui::rems(18.0))
                        .min_w_0()
                        .header(
                            CardHeader::new("Corpus explorer")
                                .description("Make room for a closer reading."),
                        )
                        .child(div().flex().justify_end().child(nav_button(
                            "open-explore",
                            "Explore texts",
                            "/explore",
                        ))),
                )),
        )
}
