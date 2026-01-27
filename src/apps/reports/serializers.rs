//! Serializers for report responses

use serde::{Deserialize, Serialize};

/// Category summary in reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySummary {
    pub category_id: i64,
    pub category_name: String,
    pub total_amount: i64,
    pub transaction_count: i32,
}

/// Monthly report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyReportResponse {
    pub year: i32,
    pub month: u32,
    pub total_income: i64,
    pub total_expense: i64,
    pub net_balance: i64,
    pub income_by_category: Vec<CategorySummary>,
    pub expense_by_category: Vec<CategorySummary>,
}

/// Yearly report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyReportResponse {
    pub year: i32,
    pub total_income: i64,
    pub total_expense: i64,
    pub net_balance: i64,
    pub monthly_summary: Vec<MonthlySummary>,
}

/// Monthly summary within yearly report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlySummary {
    pub month: u32,
    pub total_income: i64,
    pub total_expense: i64,
    pub net_balance: i64,
}

/// Category report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryReportResponse {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub categories: Vec<CategorySummary>,
    pub total_income: i64,
    pub total_expense: i64,
}
