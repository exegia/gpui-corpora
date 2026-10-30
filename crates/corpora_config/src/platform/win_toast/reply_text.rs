#[allow(unused_imports)]
use super::*;

/// Quick-reply text: the `<input id="reply">` box surfaces in
/// `ToastActivatedEventArgs::UserInput` as an `IReference<HSTRING>` box.
pub(crate) fn reply_text(args: &Ref<'_, IInspectable>) -> Option<String> {
    let event_args = args.as_ref()?.cast::<ToastActivatedEventArgs>().ok()?;
    let input = event_args.UserInput().ok()?;
    let value = input.Lookup(&HSTRING::from(REPLY_INPUT_ID)).ok()?;
    let text = value.cast::<IReference<HSTRING>>().ok()?.Value().ok()?;
    Some(text.to_string())
}
