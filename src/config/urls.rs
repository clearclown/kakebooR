//! URL configuration for kakeboor project (RESTful)
//!
//! The `url_patterns` function defines all URL patterns for this project.

use reinhardt::ServerRouter;

/// Returns the URL patterns for this project.
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        // Categories API endpoints
        .mount("/api/categories/", crate::apps::categories::url_patterns())
        // Transactions API endpoints
        .mount("/api/transactions/", crate::apps::transactions::url_patterns())
        // Reports API endpoints
        .mount("/api/reports/", crate::apps::reports::url_patterns())
}
