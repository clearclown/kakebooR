//! URL routing for transactions app

use hyper::Method;
use reinhardt_urls::prelude::ServerRouter;
use super::views;

/// Returns URL patterns for the transactions app
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        .route("/api/transactions/", Method::GET, views::list_transactions)
        .route("/api/transactions/{id}/", Method::GET, views::get_transaction)
        .route("/api/transactions/", Method::POST, views::create_transaction_view)
        .route("/api/transactions/{id}/", Method::PUT, views::update_transaction_view)
        .route("/api/transactions/{id}/", Method::DELETE, views::delete_transaction_view)
}
