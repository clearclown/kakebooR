//! URL routing for categories app

use hyper::Method;
use reinhardt_urls::prelude::ServerRouter;
use super::views;

/// Returns URL patterns for the categories app
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        .route("/api/categories/", Method::GET, views::list_categories)
        .route("/api/categories/{id}/", Method::GET, views::get_category)
        .route("/api/categories/", Method::POST, views::create_category_view)
        .route("/api/categories/{id}/", Method::PUT, views::update_category_view)
        .route("/api/categories/{id}/", Method::DELETE, views::delete_category_view)
}
