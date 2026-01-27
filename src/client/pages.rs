//! Page components
//!
//! This module defines page-level components for routing.

use reinhardt_pages::component::View;

/// Dashboard page
pub fn dashboard_page() -> View {
    crate::client::components::dashboard::dashboard_overview()
}

/// Transactions list page
pub fn transactions_page() -> View {
    crate::client::components::transactions::transactions_list()
}
