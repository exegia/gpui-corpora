#[allow(unused_imports)]
use super::*;

#[derive(Clone)]
pub struct CommandSpec {
    pub id: CommandId,
    pub title: SharedString,
    pub subtitle: SharedString,
    pub icon: IconName,
}
