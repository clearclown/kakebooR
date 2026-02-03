//! Category model definitions

use chrono::{DateTime, Utc};
use reinhardt::prelude::*;
use serde::{Deserialize, Serialize};

/// Category type representing whether it's for income or expense
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CategoryType {
    /// Income category (salary, bonus, etc.)
    Income,
    /// Expense category (food, transport, etc.)
    Expense,
}

impl std::fmt::Display for CategoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryType::Income => write!(f, "income"),
            CategoryType::Expense => write!(f, "expense"),
        }
    }
}

impl std::str::FromStr for CategoryType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "income" => Ok(CategoryType::Income),
            "expense" => Ok(CategoryType::Expense),
            _ => Err(format!("Invalid category type: {}", s)),
        }
    }
}

/// Category model for organizing transactions
#[derive(Serialize, Deserialize)]
#[model(app_label = "categories", table_name = "categories")]
pub struct Category {
    /// Unique identifier
    #[field(primary_key = true)]
    pub id: Option<i64>,

    /// Category name (e.g., "Food", "Salary")
    #[field(max_length = 100)]
    pub name: String,

    /// Type of category (Income or Expense) - stored as string
    #[field(max_length = 20)]
    pub category_type: String,

    /// Optional icon identifier for UI display
    #[field(max_length = 50, null = true)]
    pub icon: Option<String>,

    /// Optional color code (hex format, e.g., "#FF5733")
    #[field(max_length = 10, null = true)]
    pub color: Option<String>,

    /// Creation timestamp
    #[field(auto_now_add = true)]
    pub created_at: DateTime<Utc>,
}

impl Category {
    /// Gets the category type as enum
    pub fn get_category_type(&self) -> CategoryType {
        self.category_type.parse().unwrap_or(CategoryType::Expense)
    }
}
