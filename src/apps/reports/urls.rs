//! URL routing for reports app

use reinhardt::ServerRouter;

use super::views;

/// Returns URL patterns for the reports app
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        .endpoint(views::monthly_report)
        .endpoint(views::yearly_report)
        .endpoint(views::by_category_report)
}
