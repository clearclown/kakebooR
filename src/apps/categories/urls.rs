//! URL routing for categories app

use reinhardt::ServerRouter;

use super::views;

/// Returns URL patterns for the categories app
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        .endpoint(views::list_categories)
        .endpoint(views::get_category)
        .endpoint(views::create_category_view)
        .endpoint(views::update_category_view)
        .endpoint(views::delete_category_view)
}
