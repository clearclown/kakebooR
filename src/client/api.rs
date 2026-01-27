//! API client for kakebooR
//!
//! Provides functions to fetch data from the REST API.

use crate::shared::types::{
    CategoryInfo, CategoryListResponse, MonthlyReportInfo, TransactionInfo,
    TransactionListResponse,
};
use gloo_net::http::Request;

/// API base URL
const API_BASE: &str = "/api";

/// Fetch all categories
pub async fn get_categories() -> Result<Vec<CategoryInfo>, String> {
    let response = Request::get(&format!("{}/categories/", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("Failed to fetch categories: {}", response.status()));
    }

    let data: CategoryListResponse = response.json().await.map_err(|e| e.to_string())?;
    Ok(data.results)
}

/// Fetch all transactions
pub async fn get_transactions() -> Result<Vec<TransactionInfo>, String> {
    let response = Request::get(&format!("{}/transactions/", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!(
            "Failed to fetch transactions: {}",
            response.status()
        ));
    }

    let data: TransactionListResponse = response.json().await.map_err(|e| e.to_string())?;
    Ok(data.results)
}

/// Fetch monthly report
pub async fn get_monthly_report(year: i32, month: u32) -> Result<MonthlyReportInfo, String> {
    let response = Request::get(&format!(
        "{}/reports/monthly/?year={}&month={}",
        API_BASE, year, month
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("Failed to fetch report: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

/// Create a new transaction
pub async fn create_transaction(
    amount: i64,
    category_id: i64,
    description: &str,
    transaction_date: &str,
    transaction_type: &str,
) -> Result<TransactionInfo, String> {
    let body = serde_json::json!({
        "amount": amount,
        "category_id": category_id,
        "description": description,
        "transaction_date": transaction_date,
        "transaction_type": transaction_type
    });

    let response = Request::post(&format!("{}/transactions/", API_BASE))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!(
            "Failed to create transaction: {}",
            response.status()
        ));
    }

    response.json().await.map_err(|e| e.to_string())
}

/// Delete a transaction
pub async fn delete_transaction(id: i64) -> Result<(), String> {
    let response = Request::delete(&format!("{}/transactions/{}/", API_BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() && response.status() != 204 {
        return Err(format!(
            "Failed to delete transaction: {}",
            response.status()
        ));
    }

    Ok(())
}
