#[allow(unused_imports)]
use super::*;

#[derive(Debug)]
pub struct ToastResponse {
    pub tag: String,
    pub action: ResponseAction,
    pub user_text: Option<String>,
}
