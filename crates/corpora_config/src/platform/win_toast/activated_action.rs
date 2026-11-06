#[allow(unused_imports)]
use super::*;

pub(crate) fn activated_action(args: &Ref<'_, IInspectable>) -> ResponseAction {
    let arguments = args
        .as_ref()
        .and_then(|insp| insp.cast::<ToastActivatedEventArgs>().ok())
        .and_then(|event_args| event_args.Arguments().ok())
        .filter(|arguments| !arguments.is_empty());
    match arguments {
        Some(arguments) => ResponseAction::Other(arguments.to_string()),
        None => ResponseAction::Default,
    }
}
