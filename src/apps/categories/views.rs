//! Category views (API endpoints)

use reinhardt_http::{Request, Response, Result};
use validator::Validate;

use super::models::{get_all_categories, get_category_by_id, create_category, update_category, delete_category};
use super::serializers::{CreateCategoryRequest, UpdateCategoryRequest, CategoryResponse, CategoryListResponse};

/// List all categories
///
/// GET /api/categories/
pub async fn list_categories(_req: Request) -> Result<Response> {
    let categories = get_all_categories();
    let response = CategoryListResponse::new(categories);
    let json = serde_json::to_string(&response)
        .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;

    Ok(Response::ok()
        .with_header("Content-Type", "application/json")
        .with_body(json))
}

/// Get a single category by ID
///
/// GET /api/categories/{id}/
pub async fn get_category(req: Request) -> Result<Response> {
    let id: i64 = req.path_params
        .get("id")
        .ok_or_else(|| reinhardt_core::exception::Error::ParseError("Missing id parameter".to_string()))?
        .parse()
        .map_err(|_| reinhardt_core::exception::Error::ParseError("Invalid id format".to_string()))?;

    match get_category_by_id(id) {
        Some(category) => {
            let response: CategoryResponse = category.into();
            let json = serde_json::to_string(&response)
                .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
            Ok(Response::ok()
                .with_header("Content-Type", "application/json")
                .with_body(json))
        }
        None => {
            let error = serde_json::json!({"error": "Category not found"});
            let json = serde_json::to_string(&error)
                .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
            Ok(Response::not_found()
                .with_header("Content-Type", "application/json")
                .with_body(json))
        }
    }
}

/// Create a new category
///
/// POST /api/categories/
pub async fn create_category_view(req: Request) -> Result<Response> {
    // Parse request body using built-in json() method
    let create_req: CreateCategoryRequest = req.json()?;

    // Validate request
    if let Err(errors) = create_req.validate() {
        let error = serde_json::json!({"error": "Validation failed", "details": errors.to_string()});
        let json = serde_json::to_string(&error)
            .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
        return Ok(Response::bad_request()
            .with_header("Content-Type", "application/json")
            .with_body(json));
    }

    // Create category
    let category = create_category(
        create_req.name,
        create_req.category_type,
        create_req.icon,
        create_req.color,
    );

    let response: CategoryResponse = category.into();
    let json = serde_json::to_string(&response)
        .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;

    Ok(Response::created()
        .with_header("Content-Type", "application/json")
        .with_body(json))
}

/// Update an existing category
///
/// PUT /api/categories/{id}/
pub async fn update_category_view(req: Request) -> Result<Response> {
    let id: i64 = req.path_params
        .get("id")
        .ok_or_else(|| reinhardt_core::exception::Error::ParseError("Missing id parameter".to_string()))?
        .parse()
        .map_err(|_| reinhardt_core::exception::Error::ParseError("Invalid id format".to_string()))?;

    // Parse request body using built-in json() method
    let update_req: UpdateCategoryRequest = req.json()?;

    // Validate request
    if let Err(errors) = update_req.validate() {
        let error = serde_json::json!({"error": "Validation failed", "details": errors.to_string()});
        let json = serde_json::to_string(&error)
            .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
        return Ok(Response::bad_request()
            .with_header("Content-Type", "application/json")
            .with_body(json));
    }

    // Update category
    match update_category(id, update_req.name, update_req.icon, update_req.color) {
        Some(category) => {
            let response: CategoryResponse = category.into();
            let json = serde_json::to_string(&response)
                .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
            Ok(Response::ok()
                .with_header("Content-Type", "application/json")
                .with_body(json))
        }
        None => {
            let error = serde_json::json!({"error": "Category not found"});
            let json = serde_json::to_string(&error)
                .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
            Ok(Response::not_found()
                .with_header("Content-Type", "application/json")
                .with_body(json))
        }
    }
}

/// Delete a category
///
/// DELETE /api/categories/{id}/
pub async fn delete_category_view(req: Request) -> Result<Response> {
    let id: i64 = req.path_params
        .get("id")
        .ok_or_else(|| reinhardt_core::exception::Error::ParseError("Missing id parameter".to_string()))?
        .parse()
        .map_err(|_| reinhardt_core::exception::Error::ParseError("Invalid id format".to_string()))?;

    if delete_category(id) {
        Ok(Response::no_content())
    } else {
        let error = serde_json::json!({"error": "Category not found"});
        let json = serde_json::to_string(&error)
            .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
        Ok(Response::not_found()
            .with_header("Content-Type", "application/json")
            .with_body(json))
    }
}
