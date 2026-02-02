//! Transaction views (API endpoints)

use reinhardt::core::serde::json;
use reinhardt::http::ViewResult;
use reinhardt::{delete, get, post, put, Json, Path, Response, StatusCode};
use validator::Validate;

use super::models::{
    create_transaction, delete_transaction, get_all_transactions, get_transaction_by_id,
    update_transaction,
};
use super::serializers::{
    CreateTransactionRequest, TransactionListResponse, TransactionResponse, UpdateTransactionRequest,
};

/// List all transactions
///
/// GET /transactions/
#[get("/", name = "transactions_list")]
pub async fn list_transactions() -> ViewResult<Response> {
    let transactions = get_all_transactions();
    let response = TransactionListResponse::new(transactions);

    Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
}

/// Get a single transaction by ID
///
/// GET /transactions/{id}/
#[get("/{id}/", name = "transactions_get")]
pub async fn get_transaction(Path(id): Path<i64>) -> ViewResult<Response> {
    match get_transaction_by_id(id) {
        Some(transaction) => {
            let response: TransactionResponse = transaction.into();
            Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
        }
        None => Ok(Response::new(StatusCode::NOT_FOUND).with_body(
            format!(r#"{{"error": "Transaction with id {} not found"}}"#, id).into_bytes(),
        )),
    }
}

/// Create a new transaction
///
/// POST /transactions/
#[post("/", name = "transactions_create")]
pub async fn create_transaction_view(
    Json(create_req): Json<CreateTransactionRequest>,
) -> ViewResult<Response> {
    // Validate request
    create_req.validate()?;

    // Create transaction
    let transaction = create_transaction(
        create_req.amount,
        create_req.category_id,
        create_req.description,
        create_req.transaction_date,
        create_req.transaction_type,
    );

    let response: TransactionResponse = transaction.into();

    Ok(Response::new(StatusCode::CREATED).with_body(json::to_vec(&response)?))
}

/// Update an existing transaction
///
/// PUT /transactions/{id}/
#[put("/{id}/", name = "transactions_update")]
pub async fn update_transaction_view(
    Path(id): Path<i64>,
    Json(update_req): Json<UpdateTransactionRequest>,
) -> ViewResult<Response> {
    // Validate request
    update_req.validate()?;

    // Update transaction
    match update_transaction(
        id,
        update_req.amount,
        update_req.category_id,
        update_req.description,
        update_req.transaction_date,
    ) {
        Some(transaction) => {
            let response: TransactionResponse = transaction.into();
            Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
        }
        None => Ok(Response::new(StatusCode::NOT_FOUND).with_body(
            format!(r#"{{"error": "Transaction with id {} not found"}}"#, id).into_bytes(),
        )),
    }
}

/// Delete a transaction
///
/// DELETE /transactions/{id}/
#[delete("/{id}/", name = "transactions_delete")]
pub async fn delete_transaction_view(Path(id): Path<i64>) -> ViewResult<Response> {
    if delete_transaction(id) {
        Ok(Response::new(StatusCode::NO_CONTENT).with_body(Vec::new()))
    } else {
        Ok(Response::new(StatusCode::NOT_FOUND).with_body(
            format!(r#"{{"error": "Transaction with id {} not found"}}"#, id).into_bytes(),
        ))
    }
}
