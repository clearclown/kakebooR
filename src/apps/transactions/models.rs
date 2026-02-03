//! Transaction model definitions

use chrono::{DateTime, Utc};
use reinhardt::prelude::*;
use serde::{Deserialize, Serialize};

/// Transaction type representing income or expense
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    /// Income transaction (salary, bonus, etc.)
    Income,
    /// Expense transaction (food, transport, etc.)
    Expense,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Income => write!(f, "income"),
            TransactionType::Expense => write!(f, "expense"),
        }
    }
}

impl std::str::FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "income" => Ok(TransactionType::Income),
            "expense" => Ok(TransactionType::Expense),
            _ => Err(format!("Invalid transaction type: {}", s)),
        }
    }
}

/// Transaction model for recording income and expenses
#[derive(Serialize, Deserialize)]
#[model(app_label = "transactions", table_name = "transactions")]
pub struct Transaction {
    /// Unique identifier
    #[field(primary_key = true)]
    pub id: Option<i64>,

    /// Amount in yen (positive value)
    pub amount: i64,

    /// Category ID (foreign key)
    pub category_id: i64,

    /// Description or memo
    #[field(max_length = 500)]
    pub description: String,

    /// Date of the transaction (stored as timestamp)
    pub transaction_date: DateTime<Utc>,

    /// Type of transaction (Income or Expense) - stored as string
    #[field(max_length = 20)]
    pub transaction_type: String,

    /// Creation timestamp
    #[field(auto_now_add = true)]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    #[field(auto_now = true)]
    pub updated_at: DateTime<Utc>,
}

impl Transaction {
    /// Gets the transaction type as enum
    pub fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type
            .parse()
            .unwrap_or(TransactionType::Expense)
    }
}
