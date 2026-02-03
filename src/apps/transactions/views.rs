//! Transaction views (API endpoints)

use chrono::Utc;
use reinhardt::core::serde::json;
use reinhardt::http::ViewResult;
use reinhardt::Model;
use reinhardt::{delete, get, post, put, Json, Path, Response, StatusCode};
use validator::Validate;

use super::models::Transaction;
use super::serializers::{
    CreateTransactionRequest, TransactionListResponse, TransactionResponse, UpdateTransactionRequest,
};

/// List all transactions
///
/// GET /transactions/
#[get("/", name = "transactions_list")]
pub async fn list_transactions() -> ViewResult<Response> {
    let manager = Transaction::objects();
    let transactions = manager.all().all().await?;

    let response = TransactionListResponse::new(transactions);
    Ok(Response::new(StatusCode::OK).with_body(json::to_vec(&response)?))
}

/// Get a single transaction by ID
///
/// GET /transactions/{id}/
#[get("/{id}/", name = "transactions_get")]
pub async fn get_transaction(Path(id): Path<i64>) -> ViewResult<Response> {
    let manager = Transaction::objects();
    match manager.get(id).first().await? {
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
    let now = Utc::now();
    let transaction = Transaction {
        id: None,
        amount: create_req.amount,
        category_id: create_req.category_id,
        description: create_req.description,
        transaction_date: create_req.transaction_date,
        transaction_type: create_req.transaction_type.to_string(),
        created_at: now,
        updated_at: now,
    };

    let manager = Transaction::objects();
    let created = manager.create(&transaction).await?;

    let response: TransactionResponse = created.into();
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

    let manager = Transaction::objects();
    match manager.get(id).first().await? {
        Some(mut transaction) => {
            // Apply updates
            if let Some(a) = update_req.amount {
                transaction.amount = a;
            }
            if let Some(c) = update_req.category_id {
                transaction.category_id = c;
            }
            if let Some(d) = update_req.description {
                transaction.description = d;
            }
            if let Some(date) = update_req.transaction_date {
                transaction.transaction_date = date;
            }
            transaction.updated_at = Utc::now();

            let updated = manager.update(&transaction).await?;
            let response: TransactionResponse = updated.into();
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
    let manager = Transaction::objects();
    match manager.delete(id).await {
        Ok(_) => Ok(Response::new(StatusCode::NO_CONTENT).with_body(Vec::new())),
        Err(_) => Ok(Response::new(StatusCode::NOT_FOUND).with_body(
            format!(r#"{{"error": "Transaction with id {} not found"}}"#, id).into_bytes(),
        )),
    }
}
