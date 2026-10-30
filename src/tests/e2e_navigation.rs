//! End-to-end tests for navigation: sidebar page registration, deep-link
//! route parsing, and view construction.

use gpui_starter::routes::AppRoute;
use gpui_starter::sidebar::Page;

// ---------------------------------------------------------------------------
// Sidebar pages
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Route parsing (deep links)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// View construction
// ---------------------------------------------------------------------------

mod test_about_page_loads;
mod test_route_parsing;
mod test_settings_page_loads;
mod test_sidebar_pages_exist;

use test_about_page_loads::test_about_page_loads;
use test_route_parsing::test_route_parsing;
use test_settings_page_loads::test_settings_page_loads;
use test_sidebar_pages_exist::test_sidebar_pages_exist;
