#[allow(unused_imports)]
use super::*;

pub(crate) fn command(
    id: CommandId,
    title_key: &str,
    subtitle_key: &str,
    icon: IconName,
) -> CommandSpec {
    CommandSpec {
        id,
        title: crate::i18n::localize(title_key).into(),
        subtitle: crate::i18n::localize(subtitle_key).into(),
        icon,
    }
}
