//! Report views (API endpoints)

use chrono::Datelike;
use reinhardt::core::serde::json;
use reinhardt::http::ViewResult;
use reinhardt::{get, Query, Response, StatusCode};
use serde::Deserialize;
use std::collections::HashMap;

use crate::apps::categories::models::get_all_categories;
use crate::apps::transactions::models::{get_all_transactions, TransactionType};
use super::serializers::{
    CategoryReportResponse, CategorySummary, MonthlyReportResponse, MonthlySummary,
    YearlyReportResponse,
};

/// Query parameters for monthly report
#[derive(Debug, Deserialize)]
pub struct MonthlyReportQuery {
    pub year: Option<i32>,
    pub month: Option<u32>,
}

/// Query parameters for yearly report
#[derive(Debug, Deserialize)]
pub struct YearlyReportQuery {
    pub year: Option<i32>,
}

/// Query parameters for category report
#[derive(Debug, Deserialize)]
pub struct CategoryReportQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Get monthly report
///
/// GET /reports/monthly/?year=2026&month=1
#[get("/monthly/", name = "reports_monthly")]
pub async fn monthly_report(Query(params): Query<MonthlyReportQuery>) -> ViewResult<Response> {
    let year = params.year.unwrap_or_else(|| chrono::Utc::now().year());
    let month = params.month.unwrap_or_else(|| chrono::Utc::now().month());

    let transactions = get_all_transactions();
    let categories = get_all_categories();

    // Create category name lookup
    let category_names: HashMap<i64, String> = categories
        .iter()
        .map(|c| (c.id, c.name.clone()))
        .collect();

    // Filter transactions for the specified month
    let monthly_transactions: Vec<_> = transactions
        .iter()
        .filter(|t| {
            t.transaction_date.year() == year && t.transaction_date.month() == month
        })
        .collect();

    // Calculate totals
    let total_income: i64 = monthly_transactions
        .iter()
        .filter(|t| matches!(t.transaction_type, TransactionType::Income))
        .map(|t| t.amount)
        .sum();

    let total_expense: i64 = monthly_transactions
        .iter()
        .filter(|t| matches!(t.transaction_type, TransactionType::Expense))
        .map(|t| t.amount)
        .sum();

    // Group by category
    let mut income_by_category: HashMap<i64, (i64, i32)> = HashMap::new();
    let mut expense_by_category: HashMap<i64, (i64, i32)> = HashMap::new();

    for t in &monthly_transactions {
        let map = match t.transaction_type {
            TransactionType::Income => &mut income_by_category,
            TransactionType::Expense => &mut expense_by_category,
        };
        let entry = map.entry(t.category_id).or_insert((0, 0));
        entry.0 += t.amount;
        entry.1 += 1;
    }

    let income_summary: Vec<CategorySummary> = income_by_category
        .into_iter()
        .map(|(cat_id, (amount, count))| CategorySummary {
            category_id: cat_id,
            category_name: category_names
                .get(&cat_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string()),
            total_amount: amount,
            transaction_count: count,
        })
        .collect();

    let expense_summary: Vec<CategorySummary> = expense_by_category
        .into_iter()
        .map(|(cat_id, (amount, count))| CategorySummary {
            category_id: cat_id,
            category_name: category_names
                .get(&cat_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string()),
            total_amount: amount,
            transaction_count: count,
        })
        .collect();

    let response = MonthlyReportResponse {
        year,
        month,
        total_income,
        total_expense,
        net_balance: total_income - total_expense,
        income_by_category: income_summary,
        expense_by_category: expense_summary,
    };

    Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
}

/// Get yearly report
///
/// GET /reports/yearly/?year=2026
#[get("/yearly/", name = "reports_yearly")]
pub async fn yearly_report(Query(params): Query<YearlyReportQuery>) -> ViewResult<Response> {
    let year = params.year.unwrap_or_else(|| chrono::Utc::now().year());

    let transactions = get_all_transactions();

    // Filter transactions for the specified year
    let yearly_transactions: Vec<_> = transactions
        .iter()
        .filter(|t| t.transaction_date.year() == year)
        .collect();

    // Calculate yearly totals
    let total_income: i64 = yearly_transactions
        .iter()
        .filter(|t| matches!(t.transaction_type, TransactionType::Income))
        .map(|t| t.amount)
        .sum();

    let total_expense: i64 = yearly_transactions
        .iter()
        .filter(|t| matches!(t.transaction_type, TransactionType::Expense))
        .map(|t| t.amount)
        .sum();

    // Calculate monthly summaries
    let mut monthly_data: HashMap<u32, (i64, i64)> = HashMap::new();
    for month in 1..=12 {
        monthly_data.insert(month, (0, 0));
    }

    for t in &yearly_transactions {
        let month = t.transaction_date.month();
        let entry = monthly_data.entry(month).or_insert((0, 0));
        match t.transaction_type {
            TransactionType::Income => entry.0 += t.amount,
            TransactionType::Expense => entry.1 += t.amount,
        }
    }

    let monthly_summary: Vec<MonthlySummary> = (1..=12)
        .map(|month| {
            let (income, expense) = monthly_data.get(&month).copied().unwrap_or((0, 0));
            MonthlySummary {
                month,
                total_income: income,
                total_expense: expense,
                net_balance: income - expense,
            }
        })
        .collect();

    let response = YearlyReportResponse {
        year,
        total_income,
        total_expense,
        net_balance: total_income - total_expense,
        monthly_summary,
    };

    Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
}

/// Get report by category
///
/// GET /reports/by-category/?start_date=2026-01-01&end_date=2026-01-31
#[get("/by-category/", name = "reports_by_category")]
pub async fn by_category_report(Query(params): Query<CategoryReportQuery>) -> ViewResult<Response> {
    let start_date = params.start_date;
    let end_date = params.end_date;

    let transactions = get_all_transactions();
    let categories = get_all_categories();

    // Create category name lookup
    let category_names: HashMap<i64, String> = categories
        .iter()
        .map(|c| (c.id, c.name.clone()))
        .collect();

    // Filter transactions by date range if provided
    let filtered_transactions: Vec<_> = transactions
        .iter()
        .filter(|t| {
            let in_range_start = start_date
                .as_ref()
                .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                .map(|d| t.transaction_date >= d)
                .unwrap_or(true);

            let in_range_end = end_date
                .as_ref()
                .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                .map(|d| t.transaction_date <= d)
                .unwrap_or(true);

            in_range_start && in_range_end
        })
        .collect();

    // Group by category
    let mut category_totals: HashMap<i64, (i64, i32, bool)> = HashMap::new();

    for t in &filtered_transactions {
        let is_income = matches!(t.transaction_type, TransactionType::Income);
        let entry = category_totals
            .entry(t.category_id)
            .or_insert((0, 0, is_income));
        entry.0 += t.amount;
        entry.1 += 1;
    }

    let categories_summary: Vec<CategorySummary> = category_totals
        .into_iter()
        .map(|(cat_id, (amount, count, _))| CategorySummary {
            category_id: cat_id,
            category_name: category_names
                .get(&cat_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string()),
            total_amount: amount,
            transaction_count: count,
        })
        .collect();

    let total_income: i64 = filtered_transactions
        .iter()
        .filter(|t| matches!(t.transaction_type, TransactionType::Income))
        .map(|t| t.amount)
        .sum();

    let total_expense: i64 = filtered_transactions
        .iter()
        .filter(|t| matches!(t.transaction_type, TransactionType::Expense))
        .map(|t| t.amount)
        .sum();

    let response = CategoryReportResponse {
        start_date,
        end_date,
        categories: categories_summary,
        total_income,
        total_expense,
    };

    Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
}
