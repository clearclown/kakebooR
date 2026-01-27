//! Client-side router for kakebooR
//!
//! Provides client-side routing using reinhardt-pages router.

use reinhardt_pages::component::View;
use reinhardt_pages::page;
use reinhardt_pages::router::{PathPattern, Route, Router};
use std::cell::RefCell;

use super::pages;

thread_local! {
    static ROUTER: RefCell<Option<Router>> = const { RefCell::new(None) };
}

/// Initialize the global router
pub fn init_global_router() {
    let router = Router::new(vec![
        Route::new(PathPattern::new("/"), || pages::dashboard_page()),
        Route::new(PathPattern::new("/transactions"), || {
            pages::transactions_page()
        }),
        Route::new(PathPattern::new("/transactions/"), || {
            pages::transactions_page()
        }),
    ]);

    ROUTER.with(|r| {
        *r.borrow_mut() = Some(router);
    });
}

/// Access the global router
pub fn with_router<F, R>(f: F) -> R
where
    F: FnOnce(&Router) -> R,
{
    ROUTER.with(|r| {
        let router = r.borrow();
        let router = router.as_ref().expect("Router not initialized");
        f(router)
    })
}

/// 404 Not Found page
pub fn not_found_page() -> View {
    page!(|| {
        div {
            class: "container mx-auto px-4 py-8 text-center",
            h1 {
                class: "text-4xl font-bold text-gray-800 mb-4",
                "404"
            }
            p {
                class: "text-xl text-gray-600 mb-8",
                "ページが見つかりません"
            }
            a {
                href: "/",
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                "ホームに戻る"
            }
        }
    })()
}
