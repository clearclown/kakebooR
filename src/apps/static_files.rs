//! Static file serving for development
//!
//! This module provides a simple static file server for development purposes.
//! In production, use a proper web server (nginx, etc.) to serve static files.

use hyper::Method;
use reinhardt_http::{Request, Response, Result};
use reinhardt_urls::prelude::ServerRouter;
use std::path::PathBuf;

/// Serve static files from the static/ directory
pub async fn serve_static(req: Request) -> Result<Response> {
    // Get the path parameter from wildcard route
    let file_path = req.path_params
        .get("path")
        .map(|s| s.as_str())
        .unwrap_or("");

    // Prevent directory traversal attacks
    if file_path.contains("..") {
        return Ok(Response::forbidden().with_body("Forbidden"));
    }

    // Build the full path
    let full_path = PathBuf::from("static").join(file_path);

    // Try to read the file
    match tokio::fs::read(&full_path).await {
        Ok(contents) => {
            // Determine content type based on extension
            let content_type = match full_path.extension().and_then(|e| e.to_str()) {
                Some("html") => "text/html; charset=utf-8",
                Some("js") => "application/javascript; charset=utf-8",
                Some("css") => "text/css; charset=utf-8",
                Some("json") => "application/json; charset=utf-8",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("svg") => "image/svg+xml",
                Some("ico") => "image/x-icon",
                Some("woff") => "font/woff",
                Some("woff2") => "font/woff2",
                Some("ttf") => "font/ttf",
                _ => "application/octet-stream",
            };

            Ok(Response::ok()
                .with_header("Content-Type", content_type)
                .with_header("Cache-Control", "no-cache")
                .with_body(contents))
        }
        Err(_) => Ok(Response::not_found().with_body("Not Found")),
    }
}

/// Serve the index.html for the root path
pub async fn serve_index(_req: Request) -> Result<Response> {
    let full_path = PathBuf::from("static/index.html");

    match tokio::fs::read(&full_path).await {
        Ok(contents) => {
            Ok(Response::ok()
                .with_header("Content-Type", "text/html; charset=utf-8")
                .with_header("Cache-Control", "no-cache")
                .with_body(contents))
        }
        Err(_) => Ok(Response::not_found().with_body("Not Found")),
    }
}

/// URL patterns for static file serving
pub fn url_patterns() -> ServerRouter {
    ServerRouter::new()
        .route("/", Method::GET, serve_index)
        .route("/static/{*path}", Method::GET, serve_static)
}
