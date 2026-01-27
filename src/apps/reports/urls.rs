//! URL routing for reports app

use hyper::Method;
use reinhardt_urls::prelude::ServerRouter;
use super::views;

/// Returns URL patterns for the reports app
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        .route("/api/reports/monthly/", Method::GET, views::monthly_report)
        .route("/api/reports/yearly/", Method::GET, views::yearly_report)
        .route("/api/reports/by-category/", Method::GET, views::by_category_report)
}
