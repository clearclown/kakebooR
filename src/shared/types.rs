//! Shared type definitions for kakebooR
//!
//! These types are used by both WASM frontend and server.

use serde::{Deserialize, Serialize};

/// Category type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CategoryType {
    Income,
    Expense,
}

/// Transaction type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Income,
    Expense,
}

/// Category information for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub id: i64,
    pub name: String,
    pub category_type: CategoryType,
    pub icon: Option<String>,
    pub color: Option<String>,
}

/// Transaction information for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub id: i64,
    pub amount: i64,
    pub category_id: i64,
    pub description: String,
    pub transaction_date: String,
    pub transaction_type: TransactionType,
}

/// Category list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryListResponse {
    pub count: usize,
    pub results: Vec<CategoryInfo>,
}

/// Transaction list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionListResponse {
    pub count: usize,
    pub results: Vec<TransactionInfo>,
}

/// Monthly report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyReportInfo {
    pub year: i32,
    pub month: u32,
    pub total_income: i64,
    pub total_expense: i64,
    pub net_balance: i64,
}

/// Category summary in reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummaryInfo {
    pub category_id: i64,
    pub category_name: String,
    pub total_amount: i64,
    pub transaction_count: i32,
}
