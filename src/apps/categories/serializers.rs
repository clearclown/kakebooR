//! Category serializers for request/response handling

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::{Category, CategoryType};

/// Request payload for creating a new category
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    /// Category name (required, 1-100 characters)
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    /// Category type (income or expense)
    pub category_type: CategoryType,
    /// Optional icon identifier
    #[validate(length(max = 50, message = "Icon must be at most 50 characters"))]
    pub icon: Option<String>,
    /// Optional color code (hex format)
    #[validate(length(max = 7, message = "Color must be at most 7 characters"))]
    pub color: Option<String>,
}

/// Request payload for updating an existing category
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    /// Category name (optional, 1-100 characters if provided)
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    /// Optional icon identifier
    #[validate(length(max = 50, message = "Icon must be at most 50 characters"))]
    pub icon: Option<String>,
    /// Optional color code (hex format)
    #[validate(length(max = 7, message = "Color must be at most 7 characters"))]
    pub color: Option<String>,
}

/// Response payload for a category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: i64,
    pub name: String,
    pub category_type: CategoryType,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub created_at: String,
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        let category_type = category.get_category_type();
        let created_at = category.created_at.to_rfc3339();

        Self {
            id: category.id.unwrap_or(0),
            name: category.name,
            category_type,
            icon: category.icon,
            color: category.color,
            created_at,
        }
    }
}

/// Response wrapper for list of categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryListResponse {
    pub count: usize,
    pub results: Vec<CategoryResponse>,
}

impl CategoryListResponse {
    pub fn new(categories: Vec<Category>) -> Self {
        let results: Vec<CategoryResponse> = categories.into_iter().map(Into::into).collect();
        Self {
            count: results.len(),
            results,
        }
    }
}
