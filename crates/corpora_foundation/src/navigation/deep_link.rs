//! `corpora://<section>/<segments>` deep links. Parsing validates the URL and
//! yields a router path; route existence is the router's call (unknown paths
//! render the not-found page).

use percent_encoding::percent_decode_str;
use url::Url;

use crate::errors::AppError;

#[cfg(test)]
#[path = "deep_link.test/mod.rs"]
mod deep_link_test;

mod app_url_scheme;
mod invalid_segment_chars;
mod parse_deep_link;
mod to_url;
mod valid_hosts;
mod validate_path_segment;

pub use app_url_scheme::APP_URL_SCHEME;
use invalid_segment_chars::INVALID_SEGMENT_CHARS;
pub use parse_deep_link::parse_deep_link;
pub use to_url::to_url;
pub(crate) use valid_hosts::VALID_HOSTS;
use validate_path_segment::validate_path_segment;
