//! Transactions component
//!
//! Shows list of transactions and allows creating new ones.

use crate::client::api;
use crate::shared::types::{TransactionInfo, TransactionType};
use reinhardt_pages::component::View;
use reinhardt_pages::page;
use reinhardt_pages::reactive::hooks::use_state;
use reinhardt_pages::spawn::spawn_task;
use reinhardt_pages::Signal;

/// Transaction list component
pub fn transactions_list() -> View {
    let (transactions, set_transactions) = use_state(Vec::<TransactionInfo>::new());
    let (loading, set_loading) = use_state(true);
    let (error, set_error) = use_state(None::<String>);

    {
        let set_transactions = set_transactions.clone();
        let set_loading = set_loading.clone();
        let set_error = set_error.clone();

        spawn_task(async move {
            match api::get_transactions().await {
                Ok(txs) => {
                    set_transactions(txs);
                    set_loading(false);
                }
                Err(e) => {
                    set_error(Some(e));
                    set_loading(false);
                }
            }
        });
    }

    let transactions_signal = transactions.clone();
    let loading_signal = loading.clone();
    let error_signal = error.clone();

    page!(|transactions_signal: Signal<Vec<TransactionInfo>>, loading_signal: Signal<bool>, error_signal: Signal<Option<String>>| {
        div {
            class: "container mx-auto px-4 py-8",
            div {
                class: "flex justify-between items-center mb-8",
                h1 {
                    class: "text-3xl font-bold",
                    "取引一覧"
                }
                a {
                    href: "/transactions/new",
                    class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded",
                    "+ 新規取引"
                }
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
                } else if transactions_signal.get().is_empty() {
                    div {
                        class: "text-center py-8 text-gray-500",
                        p { "取引がありません" }
                        a {
                            href: "/transactions/new",
                            class: "text-blue-500 hover:underline",
                            "最初の取引を追加しましょう"
                        }
                    }
                } else {
                    div {
                        class: "bg-white shadow overflow-hidden rounded-lg",
                        table {
                            class: "min-w-full divide-y divide-gray-200",
                            thead {
                                class: "bg-gray-50",
                                tr {
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "日付"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "説明"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "種別"
                                    }
                                    th {
                                        class: "px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "金額"
                                    }
                                }
                            }
                            tbody {
                                class: "bg-white divide-y divide-gray-200",
                                { View::fragment(transactions_signal.get().iter().map(|tx| {
                                    let date = tx.transaction_date.clone();
                                    let desc = tx.description.clone();
                                    let amount = tx.amount;
                                    let is_income = matches!(tx.transaction_type, TransactionType::Income);
                                    let type_label = if is_income { "収入" } else { "支出" };
                                    let type_class = if is_income { "text-green-600" } else { "text-red-600" };
                                    let amount_class = if is_income { "text-green-600 font-semibold" } else { "text-red-600 font-semibold" };

                                    page!(|date: String, desc: String, amount: i64, type_label: &str, type_class: &str, amount_class: &str| {
                                        tr {
                                            td {
                                                class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                { date }
                                            }
                                            td {
                                                class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                { desc }
                                            }
                                            td {
                                                class: format!("px-6 py-4 whitespace-nowrap text-sm {}", type_class),
                                                { type_label }
                                            }
                                            td {
                                                class: format!("px-6 py-4 whitespace-nowrap text-sm text-right {}", amount_class),
                                                { format!("¥{}", amount) }
                                            }
                                        }
                                    })(date, desc, amount, type_label, type_class, amount_class)
                                }).collect::<Vec<_>>()) }
                            }
                        }
                    }
                }
            }
            // Back to dashboard link
            div {
                class: "mt-8",
                a {
                    href: "/",
                    class: "text-blue-500 hover:underline",
                    "← ダッシュボードに戻る"
                }
            }
        }
    })(transactions_signal, loading_signal, error_signal)
}
