//! Transaction model definitions

use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};

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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "income" => Ok(TransactionType::Income),
            "expense" => Ok(TransactionType::Expense),
            _ => Err(format!("Invalid transaction type: {}", s)),
        }
    }
}

/// Transaction model for recording income and expenses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique identifier
    pub id: i64,
    /// Amount in yen (positive value)
    pub amount: i64,
    /// Category ID (foreign key)
    pub category_id: i64,
    /// Description or memo
    pub description: String,
    /// Date of the transaction
    pub transaction_date: NaiveDate,
    /// Type of transaction (Income or Expense)
    pub transaction_type: TransactionType,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Transaction {
    /// Creates a new transaction with the given parameters
    pub fn new(
        id: i64,
        amount: i64,
        category_id: i64,
        description: String,
        transaction_date: NaiveDate,
        transaction_type: TransactionType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            amount,
            category_id,
            description,
            transaction_date,
            transaction_type,
            created_at: now,
            updated_at: now,
        }
    }
}

// In-memory storage for transactions (temporary until database integration)
use std::sync::{Arc, Mutex};
use std::sync::LazyLock;

static TRANSACTIONS: LazyLock<Arc<Mutex<Vec<Transaction>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(vec![
        Transaction::new(
            1,
            250000,
            1, // Salary category
            "Monthly salary".to_string(),
            NaiveDate::from_ymd_opt(2026, 1, 25).unwrap(),
            TransactionType::Income,
        ),
        Transaction::new(
            2,
            1500,
            2, // Food category
            "Lunch at restaurant".to_string(),
            NaiveDate::from_ymd_opt(2026, 1, 26).unwrap(),
            TransactionType::Expense,
        ),
        Transaction::new(
            3,
            500,
            3, // Transport category
            "Train fare".to_string(),
            NaiveDate::from_ymd_opt(2026, 1, 26).unwrap(),
            TransactionType::Expense,
        ),
        Transaction::new(
            4,
            3000,
            4, // Entertainment category
            "Movie tickets".to_string(),
            NaiveDate::from_ymd_opt(2026, 1, 27).unwrap(),
            TransactionType::Expense,
        ),
    ]))
});

/// Returns all transactions
pub fn get_all_transactions() -> Vec<Transaction> {
    TRANSACTIONS.lock().unwrap().clone()
}

/// Returns a transaction by ID
pub fn get_transaction_by_id(id: i64) -> Option<Transaction> {
    TRANSACTIONS.lock().unwrap().iter().find(|t| t.id == id).cloned()
}

/// Creates a new transaction and returns it
pub fn create_transaction(
    amount: i64,
    category_id: i64,
    description: String,
    transaction_date: NaiveDate,
    transaction_type: TransactionType,
) -> Transaction {
    let mut transactions = TRANSACTIONS.lock().unwrap();
    let new_id = transactions.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let transaction = Transaction::new(
        new_id,
        amount,
        category_id,
        description,
        transaction_date,
        transaction_type,
    );
    transactions.push(transaction.clone());
    transaction
}

/// Updates a transaction by ID
pub fn update_transaction(
    id: i64,
    amount: Option<i64>,
    category_id: Option<i64>,
    description: Option<String>,
    transaction_date: Option<NaiveDate>,
) -> Option<Transaction> {
    let mut transactions = TRANSACTIONS.lock().unwrap();
    if let Some(transaction) = transactions.iter_mut().find(|t| t.id == id) {
        if let Some(a) = amount {
            transaction.amount = a;
        }
        if let Some(c) = category_id {
            transaction.category_id = c;
        }
        if let Some(d) = description {
            transaction.description = d;
        }
        if let Some(date) = transaction_date {
            transaction.transaction_date = date;
        }
        transaction.updated_at = Utc::now();
        return Some(transaction.clone());
    }
    None
}

/// Deletes a transaction by ID
pub fn delete_transaction(id: i64) -> bool {
    let mut transactions = TRANSACTIONS.lock().unwrap();
    let len_before = transactions.len();
    transactions.retain(|t| t.id != id);
    transactions.len() < len_before
}

/// Returns transactions filtered by date range
pub fn get_transactions_by_date_range(start: NaiveDate, end: NaiveDate) -> Vec<Transaction> {
    TRANSACTIONS
        .lock()
        .unwrap()
        .iter()
        .filter(|t| t.transaction_date >= start && t.transaction_date <= end)
        .cloned()
        .collect()
}

/// Returns transactions filtered by category
pub fn get_transactions_by_category(category_id: i64) -> Vec<Transaction> {
    TRANSACTIONS
        .lock()
        .unwrap()
        .iter()
        .filter(|t| t.category_id == category_id)
        .cloned()
        .collect()
}

/// Returns transactions filtered by type
pub fn get_transactions_by_type(transaction_type: TransactionType) -> Vec<Transaction> {
    TRANSACTIONS
        .lock()
        .unwrap()
        .iter()
        .filter(|t| t.transaction_type == transaction_type)
        .cloned()
        .collect()
}
