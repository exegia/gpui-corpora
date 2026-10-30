//! Localization catalog guard: every `localize`/`localize_with_args` literal
//! in src/ resolves in locales/en.yml.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

mod catalog_keys;
mod is_ident;
mod localize_literals;
mod localize_literals_resolve_in_the_catalog;
mod manifest_path;
mod missing_by_design;
mod read_file;
mod rust_sources;

use catalog_keys::catalog_keys;
use is_ident::is_ident;
use localize_literals::localize_literals;
use localize_literals_resolve_in_the_catalog::localize_literals_resolve_in_the_catalog;
use manifest_path::manifest_path;
use missing_by_design::MISSING_BY_DESIGN;
use read_file::read_file;
use rust_sources::rust_sources;
