//! URL routing for transactions app

use reinhardt::ServerRouter;

use super::views;

/// Returns URL patterns for the transactions app
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        .endpoint(views::list_transactions)
        .endpoint(views::get_transaction)
        .endpoint(views::create_transaction_view)
        .endpoint(views::update_transaction_view)
        .endpoint(views::delete_transaction_view)
}
