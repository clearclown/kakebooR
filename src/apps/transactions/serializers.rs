//! Transaction serializers for request/response handling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::{Transaction, TransactionType};

/// Request payload for creating a new transaction
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTransactionRequest {
    /// Amount in yen (must be positive)
    #[validate(range(min = 1, message = "Amount must be positive"))]
    pub amount: i64,
    /// Category ID
    pub category_id: i64,
    /// Description or memo (max 500 characters)
    #[validate(length(max = 500, message = "Description must be at most 500 characters"))]
    pub description: String,
    /// Date of the transaction (ISO 8601 format)
    pub transaction_date: DateTime<Utc>,
    /// Type of transaction (income or expense)
    pub transaction_type: TransactionType,
}

/// Request payload for updating an existing transaction
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTransactionRequest {
    /// Amount in yen (optional, must be positive if provided)
    #[validate(range(min = 1, message = "Amount must be positive"))]
    pub amount: Option<i64>,
    /// Category ID (optional)
    pub category_id: Option<i64>,
    /// Description or memo (optional, max 500 characters if provided)
    #[validate(length(max = 500, message = "Description must be at most 500 characters"))]
    pub description: Option<String>,
    /// Date of the transaction (optional)
    pub transaction_date: Option<DateTime<Utc>>,
}

/// Response payload for a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: i64,
    pub amount: i64,
    pub category_id: i64,
    pub description: String,
    pub transaction_date: String,
    pub transaction_type: TransactionType,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Transaction> for TransactionResponse {
    fn from(transaction: Transaction) -> Self {
        let transaction_type = transaction.get_transaction_type();
        let transaction_date = transaction.transaction_date.format("%Y-%m-%d").to_string();
        let created_at = transaction.created_at.to_rfc3339();
        let updated_at = transaction.updated_at.to_rfc3339();

        Self {
            id: transaction.id.unwrap_or(0),
            amount: transaction.amount,
            category_id: transaction.category_id,
            description: transaction.description,
            transaction_date,
            transaction_type,
            created_at,
            updated_at,
        }
    }
}

/// Response wrapper for list of transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionListResponse {
    pub count: usize,
    pub results: Vec<TransactionResponse>,
}

impl TransactionListResponse {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let results: Vec<TransactionResponse> =
            transactions.into_iter().map(Into::into).collect();
        Self {
            count: results.len(),
            results,
        }
    }
}

/// Summary response for aggregated transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub total_income: i64,
    pub total_expense: i64,
    pub balance: i64,
    pub transaction_count: usize,
}

impl TransactionSummary {
    pub fn from_transactions(transactions: &[Transaction]) -> Self {
        let total_income: i64 = transactions
            .iter()
            .filter(|t| t.get_transaction_type() == TransactionType::Income)
            .map(|t| t.amount)
            .sum();

        let total_expense: i64 = transactions
            .iter()
            .filter(|t| t.get_transaction_type() == TransactionType::Expense)
            .map(|t| t.amount)
            .sum();

        Self {
            total_income,
            total_expense,
            balance: total_income - total_expense,
            transaction_count: transactions.len(),
        }
    }
}
