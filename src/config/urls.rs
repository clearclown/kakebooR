//! URL configuration for kakeboor project (RESTful)
//!
//! The `routes` function defines all URL patterns for this project.

use reinhardt_urls::prelude::ServerRouter;
use crate::apps::categories;
use crate::apps::transactions;
use crate::apps::reports;
use crate::apps::static_files;

/// Returns the URL patterns for this project.
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        // Categories API endpoints
        .mount("/", categories::url_patterns())
        // Transactions API endpoints
        .mount("/", transactions::url_patterns())
        // Reports API endpoints
        .mount("/", reports::url_patterns())
        // Static files (for development)
        .mount("/", static_files::url_patterns())
}
