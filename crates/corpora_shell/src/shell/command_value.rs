#[allow(unused_imports)]
use super::*;

pub(crate) fn command_value(id: crate::commands::CommandId) -> String {
    format!("{id:?}")
}
