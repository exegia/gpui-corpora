#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SessionState {
    #[default]
    SignedOut,
    SigningIn,
    SignedIn {
        account_label: String,
    },
    Error(String),
}
