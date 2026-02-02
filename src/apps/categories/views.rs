//! Category views (API endpoints)

use reinhardt::core::serde::json;
use reinhardt::http::ViewResult;
use reinhardt::{delete, get, post, put, Json, Path, Response, StatusCode};
use validator::Validate;

use super::models::{
    create_category, delete_category, get_all_categories, get_category_by_id, update_category,
};
use super::serializers::{
    CategoryListResponse, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest,
};

/// List all categories
///
/// GET /categories/
#[get("/", name = "categories_list")]
pub async fn list_categories() -> ViewResult<Response> {
    let categories = get_all_categories();
    let response = CategoryListResponse::new(categories);

    Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
}

/// Get a single category by ID
///
/// GET /categories/{id}/
#[get("/{id}/", name = "categories_get")]
pub async fn get_category(Path(id): Path<i64>) -> ViewResult<Response> {
    match get_category_by_id(id) {
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
    let category = create_category(
        create_req.name,
        create_req.category_type,
        create_req.icon,
        create_req.color,
    );

    let response: CategoryResponse = category.into();

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

    // Update category
    match update_category(id, update_req.name, update_req.icon, update_req.color) {
        Some(category) => {
            let response: CategoryResponse = category.into();
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
    if delete_category(id) {
        Ok(Response::new(StatusCode::NO_CONTENT).with_body(Vec::new()))
    } else {
        Ok(Response::new(StatusCode::NOT_FOUND).with_body(
            format!(r#"{{"error": "Category with id {} not found"}}"#, id).into_bytes(),
        ))
    }
}
