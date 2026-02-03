//! Category views (API endpoints)

use chrono::Utc;
use reinhardt::core::serde::json;
use reinhardt::http::ViewResult;
use reinhardt::Model;
use reinhardt::{delete, get, post, put, Json, Path, Response, StatusCode};
use validator::Validate;

use super::models::Category;
use super::serializers::{
    CategoryListResponse, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest,
};

/// List all categories
///
/// GET /categories/
#[get("/", name = "categories_list")]
pub async fn list_categories() -> ViewResult<Response> {
    let manager = Category::objects();
    let categories = manager.all().all().await?;

    let response = CategoryListResponse::new(categories);
    Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
}

/// Get a single category by ID
///
/// GET /categories/{id}/
#[get("/{id}/", name = "categories_get")]
pub async fn get_category(Path(id): Path<i64>) -> ViewResult<Response> {
    let manager = Category::objects();
    match manager.get(id).first().await? {
        Some(category) => {
            let response: CategoryResponse = category.into();
            Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
        }
        None => Ok(Response::new(StatusCode::NOT_FOUND).with_body(
            format!(r#"{{"error": "Category with id {} not found"}}"#, id).into_bytes(),
        )),
    }
}

/// Create a new category
///
/// POST /categories/
#[post("/", name = "categories_create")]
pub async fn create_category_view(
    Json(create_req): Json<CreateCategoryRequest>,
) -> ViewResult<Response> {
    // Validate request
    create_req.validate()?;

    // Create category
    let now = Utc::now();
    let category = Category {
        id: None,
        name: create_req.name,
        category_type: create_req.category_type.to_string(),
        icon: create_req.icon,
        color: create_req.color,
        created_at: now,
    };

    let manager = Category::objects();
    let created = manager.create(&category).await?;

    let response: CategoryResponse = created.into();
    Ok(Response::new(StatusCode::CREATED).with_body(json::to_vec(&response)?))
}

/// Update an existing category
///
/// PUT /categories/{id}/
#[put("/{id}/", name = "categories_update")]
pub async fn update_category_view(
    Path(id): Path<i64>,
    Json(update_req): Json<UpdateCategoryRequest>,
) -> ViewResult<Response> {
    // Validate request
    update_req.validate()?;

    let manager = Category::objects();
    match manager.get(id).first().await? {
        Some(mut category) => {
            // Apply updates
            if let Some(n) = update_req.name {
                category.name = n;
            }
            if let Some(i) = update_req.icon {
                category.icon = Some(i);
            }
            if let Some(c) = update_req.color {
                category.color = Some(c);
            }

            let updated = manager.update(&category).await?;
            let response: CategoryResponse = updated.into();
            Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
        }
        None => Ok(Response::new(StatusCode::NOT_FOUND).with_body(
            format!(r#"{{"error": "Category with id {} not found"}}"#, id).into_bytes(),
        )),
    }
}

/// Delete a category
///
/// DELETE /categories/{id}/
#[delete("/{id}/", name = "categories_delete")]
pub async fn delete_category_view(Path(id): Path<i64>) -> ViewResult<Response> {
    let manager = Category::objects();
    match manager.delete(id).await {
        Ok(_) => Ok(Response::new(StatusCode::NO_CONTENT).with_body(Vec::new())),
        Err(_) => Ok(Response::new(StatusCode::NOT_FOUND).with_body(
            format!(r#"{{"error": "Category with id {} not found"}}"#, id).into_bytes(),
        )),
    }
}
