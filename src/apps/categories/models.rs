//! Category model definitions

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "income" => Ok(CategoryType::Income),
            "expense" => Ok(CategoryType::Expense),
            _ => Err(format!("Invalid category type: {}", s)),
        }
    }
}

/// Category model for organizing transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// Unique identifier
    pub id: i64,
    /// Category name (e.g., "Food", "Salary")
    pub name: String,
    /// Type of category (Income or Expense)
    pub category_type: CategoryType,
    /// Optional icon identifier for UI display
    pub icon: Option<String>,
    /// Optional color code (hex format, e.g., "#FF5733")
    pub color: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Category {
    /// Creates a new category with the given parameters
    pub fn new(id: i64, name: String, category_type: CategoryType) -> Self {
        Self {
            id,
            name,
            category_type,
            icon: None,
            color: None,
            created_at: Utc::now(),
        }
    }

    /// Sets the icon for the category
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the color for the category
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

// In-memory storage for categories (temporary until database integration)
use std::sync::{Arc, Mutex};
use std::sync::LazyLock;

static CATEGORIES: LazyLock<Arc<Mutex<Vec<Category>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(vec![
        Category::new(1, "Salary".to_string(), CategoryType::Income)
            .with_icon("wallet")
            .with_color("#4CAF50"),
        Category::new(2, "Food".to_string(), CategoryType::Expense)
            .with_icon("restaurant")
            .with_color("#FF5722"),
        Category::new(3, "Transport".to_string(), CategoryType::Expense)
            .with_icon("directions_car")
            .with_color("#2196F3"),
        Category::new(4, "Entertainment".to_string(), CategoryType::Expense)
            .with_icon("movie")
            .with_color("#9C27B0"),
    ]))
});

/// Returns all categories
pub fn get_all_categories() -> Vec<Category> {
    CATEGORIES.lock().unwrap().clone()
}

/// Returns a category by ID
pub fn get_category_by_id(id: i64) -> Option<Category> {
    CATEGORIES.lock().unwrap().iter().find(|c| c.id == id).cloned()
}

/// Creates a new category and returns it
pub fn create_category(name: String, category_type: CategoryType, icon: Option<String>, color: Option<String>) -> Category {
    let mut categories = CATEGORIES.lock().unwrap();
    let new_id = categories.iter().map(|c| c.id).max().unwrap_or(0) + 1;
    let mut category = Category::new(new_id, name, category_type);
    if let Some(i) = icon {
        category = category.with_icon(i);
    }
    if let Some(c) = color {
        category = category.with_color(c);
    }
    categories.push(category.clone());
    category
}

/// Updates a category by ID
pub fn update_category(id: i64, name: Option<String>, icon: Option<String>, color: Option<String>) -> Option<Category> {
    let mut categories = CATEGORIES.lock().unwrap();
    if let Some(category) = categories.iter_mut().find(|c| c.id == id) {
        if let Some(n) = name {
            category.name = n;
        }
        if let Some(i) = icon {
            category.icon = Some(i);
        }
        if let Some(c) = color {
            category.color = Some(c);
        }
        return Some(category.clone());
    }
    None
}

/// Deletes a category by ID
pub fn delete_category(id: i64) -> bool {
    let mut categories = CATEGORIES.lock().unwrap();
    let len_before = categories.len();
    categories.retain(|c| c.id != id);
    categories.len() < len_before
}
