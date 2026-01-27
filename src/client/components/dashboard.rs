//! Dashboard component
//!
//! Shows overview of income, expenses, and balance.

use crate::client::api;
use crate::shared::types::MonthlyReportInfo;
use reinhardt_pages::component::View;
use reinhardt_pages::page;
use reinhardt_pages::reactive::hooks::use_state;
use reinhardt_pages::spawn::spawn_task;
use reinhardt_pages::Signal;

/// Dashboard overview component
pub fn dashboard_overview() -> View {
    let (report, set_report) = use_state(None::<MonthlyReportInfo>);
    let (loading, set_loading) = use_state(true);
    let (error, set_error) = use_state(None::<String>);

    // Get current year and month
    let now = chrono::Utc::now();
    let year = now.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
    let month = now.format("%m").to_string().parse::<u32>().unwrap_or(1);

    {
        let set_report = set_report.clone();
        let set_loading = set_loading.clone();
        let set_error = set_error.clone();

        spawn_task(async move {
            match api::get_monthly_report(year, month).await {
                Ok(r) => {
                    set_report(Some(r));
                    set_loading(false);
                }
                Err(e) => {
                    set_error(Some(e));
                    set_loading(false);
                }
            }
        });
    }

    let report_signal = report.clone();
    let loading_signal = loading.clone();
    let error_signal = error.clone();

    page!(|report_signal: Signal<Option<MonthlyReportInfo>>, loading_signal: Signal<bool>, error_signal: Signal<Option<String>>| {
        div {
            class: "container mx-auto px-4 py-8",
            h1 {
                class: "text-3xl font-bold mb-8 text-center",
                "家計簿ダッシュボード"
            }
            watch {
                if loading_signal.get() {
                    div {
                        class: "text-center py-8",
                        div {
                            class: "inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500",
                        }
                        p {
                            class: "mt-2 text-gray-600",
                            "読み込み中..."
                        }
                    }
                } else if let Some(err) = error_signal.get() {
                    div {
                        class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
                        { format!("エラー: {}", err) }
                    }
                } else if let Some(r) = report_signal.get() {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                        // Income card
                        div {
                            class: "bg-green-100 rounded-lg p-6 shadow",
                            h2 {
                                class: "text-lg font-semibold text-green-800",
                                "収入"
                            }
                            p {
                                class: "text-3xl font-bold text-green-600 mt-2",
                                { format!("¥{}", format_amount(r.total_income)) }
                            }
                        }
                        // Expense card
                        div {
                            class: "bg-red-100 rounded-lg p-6 shadow",
                            h2 {
                                class: "text-lg font-semibold text-red-800",
                                "支出"
                            }
                            p {
                                class: "text-3xl font-bold text-red-600 mt-2",
                                { format!("¥{}", format_amount(r.total_expense)) }
                            }
                        }
                        // Balance card
                        div {
                            class: if r.net_balance >= 0 { "bg-blue-100 rounded-lg p-6 shadow" } else { "bg-yellow-100 rounded-lg p-6 shadow" },
                            h2 {
                                class: if r.net_balance >= 0 { "text-lg font-semibold text-blue-800" } else { "text-lg font-semibold text-yellow-800" },
                                "収支"
                            }
                            p {
                                class: if r.net_balance >= 0 { "text-3xl font-bold text-blue-600 mt-2" } else { "text-3xl font-bold text-yellow-600 mt-2" },
                                { format!("¥{}", format_amount(r.net_balance)) }
                            }
                        }
                    }
                }
            }
            // Navigation links
            div {
                class: "mt-8 flex justify-center space-x-4",
                a {
                    href: "/transactions",
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    "取引一覧"
                }
                a {
                    href: "/transactions/new",
                    class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded",
                    "新規取引"
                }
            }
        }
    })(report_signal, loading_signal, error_signal)
}

/// Format amount with thousand separators
fn format_amount(amount: i64) -> String {
    let abs_amount = amount.abs();
    let formatted: String = abs_amount
        .to_string()
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                format!(",{}", c)
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    if amount < 0 {
        format!("-{}", formatted)
    } else {
        formatted
    }
}
