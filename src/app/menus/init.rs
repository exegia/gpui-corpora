#[allow(unused_imports)]
use super::*;

pub fn init(title: impl Into<SharedString>, cx: &mut App) {
    let title: SharedString = title.into();
    cx.set_menus(build_menus(title.clone(), cx));

    let rebuild = move |cx: &mut App| cx.set_menus(build_menus(title.clone(), cx));
    cx.observe_global::<Theme>({
        let rebuild = rebuild.clone();
        move |cx| rebuild(cx)
    })
    .detach();
    cx.observe_global::<LocaleState>({
        let rebuild = rebuild.clone();
        move |cx| rebuild(cx)
    })
    .detach();
    cx.observe_global::<crate::undo_stack::UndoState>(move |cx| rebuild(cx))
        .detach();
}
