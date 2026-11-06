//! Toast identity for unpackaged Windows apps. Windows drops any toast sent
//! under an AppUserModelID that resolves to no app identity — no MSIX
//! registration and no Start Menu shortcut carrying `System.AppUserModel.ID`.
//! `ToastNotifier::Show` still reports success, so the drop is silent.
//! [`ensure_toast_identity`] installs that shortcut for the running exe
//! (refreshing it when the exe moves, as dev builds do between target dirs)
//! and sets the process AUMID. Both steps are idempotent and best-effort.

use std::path::{Path, PathBuf};

use windows::Win32::Foundation::{E_OUTOFMEMORY, PROPERTYKEY, RPC_E_CHANGED_MODE};
use windows::Win32::Storage::FileSystem::{GetLongPathNameW, WIN32_FIND_DATAW};
use windows::Win32::System::Com::StructuredStorage::{PROPVARIANT, PropVariantClear};
use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoTaskMemAlloc,
    CoTaskMemFree, CoUninitialize, IPersistFile, STGM_READ,
};
use windows::Win32::System::Variant::VT_LPWSTR;
use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;
use windows::Win32::UI::Shell::{
    FOLDERID_Programs, IShellLinkW, KNOWN_FOLDER_FLAG, SHGetKnownFolderPath,
    SetCurrentProcessExplicitAppUserModelID,
};
use windows::core::{GUID, HSTRING, Interface, PCWSTR, PWSTR};

#[cfg(test)]
#[path = "toast_identity.test/mod.rs"]
mod toast_identity_test;

mod clsid_shell_link;
mod comparable;
mod ensure_shortcut_at;
mod ensure_toast_identity;
mod load_shortcut;
mod log;
mod long_form;
mod pkey_app_user_model_id;
mod programs_dir;
mod propvariant_from_str;
mod propvariant_string;
mod pwstr_to_string;
mod same_path;
mod shortcut_matches;
mod shortcut_name;
mod with_com;
mod write_shortcut;

use clsid_shell_link::CLSID_SHELL_LINK;
use comparable::comparable;
pub(crate) use ensure_shortcut_at::ensure_shortcut_at;
pub use ensure_toast_identity::ensure_toast_identity;
use load_shortcut::load_shortcut;
use log::LOG;
use long_form::long_form;
use pkey_app_user_model_id::PKEY_APP_USER_MODEL_ID;
use programs_dir::programs_dir;
use propvariant_from_str::propvariant_from_str;
use propvariant_string::propvariant_string;
use pwstr_to_string::pwstr_to_string;
pub(crate) use same_path::same_path;
use shortcut_matches::shortcut_matches;
use shortcut_name::SHORTCUT_NAME;
use with_com::with_com;
use write_shortcut::write_shortcut;
