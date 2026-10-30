#[allow(unused_imports)]
use super::*;

pub(crate) struct Migration {
    pub(crate) version: u32,
    pub(crate) name: &'static str,
    pub(crate) up_sql: &'static str,
}
