use corpora_ui::ely::{
    motion,
    primitives::{Icon, Tooltip},
    theme::IconSize,
};
use gpui::{Animation, AnimationExt, SharedString, px};

use super::*;

/// The selected item's indicator; Ely's `ActivityBar` draws 2px.
const MARK_WIDTH: gpui::Pixels = px(3.0);

const ITEMS: [(&str, IconName, &str); 3] = [
    (navigation::HOME, IconName::House, "Home"),
    (navigation::EXPLORE, IconName::Compass, "Explore"),
    (navigation::PROJECTS, IconName::Folder, "Projects"),
];
const FOOTER: [(&str, IconName, &str); 1] = [(navigation::PROFILE, IconName::User, "Profile")];

/// The window's icon rail. Ely's `ActivityBar` paints its selection in the text
/// color with a fixed indicator, so this rail follows it with the accent.
pub(super) fn activity(
    selected: &str,
    workspace: WeakEntity<Workspace>,
    cx: &App,
) -> impl IntoElement + use<> {
    let theme = cx.theme();
    let colors = &theme.colors;
    let side = theme.rail_width();
    let mark = theme.icon_size(IconSize::Lg);
    let grow = motion::duration(motion::BASE, cx);
    let entry = |(path, icon, label): (&'static str, IconName, &'static str)| {
        let is_selected = selected == path;
        let group = SharedString::from(format!("activity-{path}"));
        let color = if is_selected {
            colors.accent
        } else {
            colors.fg_subtle
        };
        let owner = workspace.clone();
        div()
            .id(path)
            .group(group.clone())
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .size(side)
            .cursor_pointer()
            .tooltip(Tooltip::text(label))
            .on_click(move |_, _, cx| {
                owner.update(cx, |state, cx| state.navigate(path, cx)).ok();
            })
            .child(
                Icon::new(icon)
                    .size(IconSize::Lg)
                    .color(color)
                    .group_hover_color(group, if is_selected { color } else { colors.fg }),
            )
            .when(is_selected, |entry| {
                entry.child(
                    div()
                        .absolute()
                        .left_0()
                        .top_0()
                        .bottom_0()
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .h(mark)
                                .w(MARK_WIDTH)
                                .bg(colors.accent)
                                .with_animation(
                                    SharedString::from(format!("activity-mark-{path}")),
                                    Animation::new(grow).with_easing(motion::ease_out_cubic),
                                    move |line, t| line.h(mark * t),
                                ),
                        ),
                )
            })
    };
    div()
        .id("activity-bar")
        .flex()
        .flex_col()
        .flex_none()
        .justify_between()
        .w(side)
        .h_full()
        .py_1()
        .child(div().flex().flex_col().children(ITEMS.map(entry)))
        .child(div().flex().flex_col().children(FOOTER.map(entry)))
}
