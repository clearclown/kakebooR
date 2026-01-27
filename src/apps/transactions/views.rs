//! Transaction views (API endpoints)

use reinhardt_http::{Request, Response, Result};
use validator::Validate;

use super::models::{
    get_all_transactions, get_transaction_by_id, create_transaction,
    update_transaction, delete_transaction,
};
use super::serializers::{
    CreateTransactionRequest, UpdateTransactionRequest,
    TransactionResponse, TransactionListResponse,
};

/// List all transactions
///
/// GET /api/transactions/
pub async fn list_transactions(_req: Request) -> Result<Response> {
    let transactions = get_all_transactions();
    let response = TransactionListResponse::new(transactions);
    let json = serde_json::to_string(&response)
        .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;

    Ok(Response::ok()
        .with_header("Content-Type", "application/json")
        .with_body(json))
}

/// Get a single transaction by ID
///
/// GET /api/transactions/{id}/
pub async fn get_transaction(req: Request) -> Result<Response> {
    let id: i64 = req.path_params
        .get("id")
        .ok_or_else(|| reinhardt_core::exception::Error::ParseError("Missing id parameter".to_string()))?
        .parse()
        .map_err(|_| reinhardt_core::exception::Error::ParseError("Invalid id format".to_string()))?;

    match get_transaction_by_id(id) {
        Some(transaction) => {
            let response: TransactionResponse = transaction.into();
            let json = serde_json::to_string(&response)
                .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
            Ok(Response::ok()
                .with_header("Content-Type", "application/json")
                .with_body(json))
        }
        None => {
            let error = serde_json::json!({"error": "Transaction not found"});
            let json = serde_json::to_string(&error)
                .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
            Ok(Response::not_found()
                .with_header("Content-Type", "application/json")
                .with_body(json))
        }
    }
}

/// Create a new transaction
///
/// POST /api/transactions/
pub async fn create_transaction_view(req: Request) -> Result<Response> {
    // Parse request body using built-in json() method
    let create_req: CreateTransactionRequest = req.json()?;

    // Validate request
    if let Err(errors) = create_req.validate() {
        let error = serde_json::json!({"error": "Validation failed", "details": errors.to_string()});
        let json = serde_json::to_string(&error)
            .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
        return Ok(Response::bad_request()
            .with_header("Content-Type", "application/json")
            .with_body(json));
    }

    // Create transaction
    let transaction = create_transaction(
        create_req.amount,
        create_req.category_id,
        create_req.description,
        create_req.transaction_date,
        create_req.transaction_type,
    );

    let response: TransactionResponse = transaction.into();
    let json = serde_json::to_string(&response)
        .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;

    Ok(Response::created()
        .with_header("Content-Type", "application/json")
        .with_body(json))
}

/// Update an existing transaction
///
/// PUT /api/transactions/{id}/
pub async fn update_transaction_view(req: Request) -> Result<Response> {
    let id: i64 = req.path_params
        .get("id")
        .ok_or_else(|| reinhardt_core::exception::Error::ParseError("Missing id parameter".to_string()))?
        .parse()
        .map_err(|_| reinhardt_core::exception::Error::ParseError("Invalid id format".to_string()))?;

    // Parse request body using built-in json() method
    let update_req: UpdateTransactionRequest = req.json()?;

    // Validate request
    if let Err(errors) = update_req.validate() {
        let error = serde_json::json!({"error": "Validation failed", "details": errors.to_string()});
        let json = serde_json::to_string(&error)
            .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
        return Ok(Response::bad_request()
            .with_header("Content-Type", "application/json")
            .with_body(json));
    }

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
            let json = serde_json::to_string(&response)
                .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
            Ok(Response::ok()
                .with_header("Content-Type", "application/json")
                .with_body(json))
        }
        None => {
            let error = serde_json::json!({"error": "Transaction not found"});
            let json = serde_json::to_string(&error)
                .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
            Ok(Response::not_found()
                .with_header("Content-Type", "application/json")
                .with_body(json))
        }
    }
}

/// Delete a transaction
///
/// DELETE /api/transactions/{id}/
pub async fn delete_transaction_view(req: Request) -> Result<Response> {
    let id: i64 = req.path_params
        .get("id")
        .ok_or_else(|| reinhardt_core::exception::Error::ParseError("Missing id parameter".to_string()))?
        .parse()
        .map_err(|_| reinhardt_core::exception::Error::ParseError("Invalid id format".to_string()))?;

    if delete_transaction(id) {
        Ok(Response::no_content())
    } else {
        let error = serde_json::json!({"error": "Transaction not found"});
        let json = serde_json::to_string(&error)
            .map_err(|e| reinhardt_core::exception::Error::Internal(e.to_string()))?;
        Ok(Response::not_found()
            .with_header("Content-Type", "application/json")
            .with_body(json))
    }
}
