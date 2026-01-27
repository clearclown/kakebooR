# CLAUDE.md

## Purpose

このファイルはkakebooRプロジェクト固有の開発指針を記載しています。Reinhardtフレームワークを使用した家計簿アプリの開発ルールとコード品質基準を定義します。

---

## Project Overview

**プロジェクト名**: kakebooR（家計簿 + R for Rust/Reinhardt）

**概要**: Rust製の家計簿Webアプリケーション。Reinhardtフレームワークを使用してREST APIを構築。

詳細は README.md を参照。

---

## Tech Stack

- **Language**: Rust 2024 Edition
- **Framework**: Reinhardt Web Framework
- **Module System**: Rust 2024 edition（`mod.rs` は使用禁止）
- **Database**: PostgreSQL + Reinhardt ORM
- **Testing**: Rust's built-in framework + TestContainers
- **Build**: Cargo

---

## Reinhardt Framework Reference

Reinhardtの詳細なドキュメントは `docs/repos/reinhardt-web/` に配置:

- **Getting Started**: `docs/repos/reinhardt-web/docs/GETTING_STARTED.md`
- **REST Tutorial**: `docs/repos/reinhardt-web/docs/tutorials/en/rest/`
- **Basis Tutorial**: `docs/repos/reinhardt-web/docs/tutorials/en/basis/`
- **Feature Flags**: `docs/repos/reinhardt-web/docs/FEATURE_FLAGS.md`
- **Module System**: `docs/repos/reinhardt-web/docs/MODULE_SYSTEM.md`
- **Testing Standards**: `docs/repos/reinhardt-web/docs/TESTING_STANDARDS.md`

---

## Critical Rules

### Module System

**MUST use `module.rs` + `module/` directory structure (Rust 2024 Edition)**
**NEVER use `mod.rs` files** (deprecated)

### Code Style

**Key Requirements:**
- **ALL code comments MUST be written in English**
- MINIMIZE `.to_string()` calls - prefer borrowing
- DELETE obsolete code immediately
- Mark ALL placeholders with `todo!()` or `// TODO:` comment

### Reinhardt Patterns

**アプリ構成（Django風）:**
```
apps/
└── <app_name>/
    ├── models.rs      # データモデル定義
    ├── views.rs       # APIエンドポイント
    ├── serializers.rs # シリアライゼーション
    └── urls.rs        # URLルーティング
```

**Model定義パターン:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub amount: i64,          // 金額（整数で管理、小数点以下は別途対応）
    pub category_id: i64,
    pub description: String,
    pub transaction_date: chrono::NaiveDate,
    pub transaction_type: TransactionType,  // Income or Expense
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

**View定義パターン（HTTP decorators使用）:**
```rust
use reinhardt::prelude::*;
use reinhardt::{get, post, put, delete};

#[get("/transactions", name = "list_transactions")]
pub async fn list_transactions(
    #[inject] conn: Arc<DatabaseConnection>,
) -> Result<Response> {
    // Implementation
}

#[post("/transactions", name = "create_transaction")]
pub async fn create_transaction(
    request: Request,
    #[inject] conn: Arc<DatabaseConnection>,
) -> Result<Response> {
    // Implementation
}
```

**URLルーティングパターン:**
```rust
use reinhardt::routers::UnifiedRouter;

pub fn url_patterns() -> UnifiedRouter {
    UnifiedRouter::new()
        .with_namespace("transactions")
        .endpoint(views::list_transactions)
        .endpoint(views::create_transaction)
        // ...
}
```

### Testing

**Core Principles:**
- NO skeleton tests (all tests MUST have meaningful assertions)
- Unit tests: Test single component behavior
- Integration tests: Test API endpoints with TestContainers

### File Management

**Critical Rules:**
- **NEVER** save temp files to project directory (use `/tmp`)
- **IMMEDIATELY** delete temp files when no longer needed
- NO relative paths beyond `../`

### Git Workflow

**Commit Policy:**
- **NEVER** commit without explicit user instruction
- **NEVER** push without explicit user instruction
- Split commits by specific intent

---

## Common Commands

**Check & Build:**
```bash
cargo check --all-features
cargo build --all-features
```

**Testing:**
```bash
cargo test --all-features
```

**Code Quality:**
```bash
cargo fmt --check
cargo clippy --all-features
```

**Development Server:**
```bash
cargo run --bin runserver
```

**Management Commands:**
```bash
# Create a new app
cargo run --bin manage startapp <app_name> --template-type restful

# Create migrations
cargo run --bin manage makemigrations

# Apply migrations
cargo run --bin manage migrate
```

---

## Data Models

### Transaction（収支記録）

| Field | Type | Description |
|-------|------|-------------|
| id | i64 | Primary key |
| amount | i64 | 金額（円） |
| category_id | i64 | カテゴリID（FK） |
| description | String | 説明・メモ |
| transaction_date | NaiveDate | 取引日 |
| transaction_type | Enum | Income / Expense |
| created_at | DateTime | 作成日時 |
| updated_at | DateTime | 更新日時 |

### Category（カテゴリ）

| Field | Type | Description |
|-------|------|-------------|
| id | i64 | Primary key |
| name | String | カテゴリ名 |
| category_type | Enum | Income / Expense |
| icon | Option<String> | アイコン識別子 |
| color | Option<String> | カラーコード |
| created_at | DateTime | 作成日時 |

### TransactionType（取引種別）

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Income,   // 収入
    Expense,  // 支出
}
```

---

## Development Phases

### Phase 1: Project Setup
- [ ] Reinhardtプロジェクト初期化
- [ ] 基本ディレクトリ構成作成
- [ ] データベース設定

### Phase 2: Core Models
- [ ] Category model実装
- [ ] Transaction model実装
- [ ] マイグレーション作成

### Phase 3: REST API
- [ ] Categories CRUD API
- [ ] Transactions CRUD API
- [ ] バリデーション実装

### Phase 4: Reports
- [ ] 月次集計API
- [ ] カテゴリ別集計API
- [ ] 年次集計API

### Phase 5: Testing & Documentation
- [ ] Unit tests
- [ ] Integration tests
- [ ] API documentation

---

## Quick Reference

### MUST DO
- Write ALL code comments in English
- Use `module.rs` + `module/` directory (NO `mod.rs`)
- Follow Reinhardt patterns for models, views, serializers
- Use HTTP decorators (`#[get]`, `#[post]`, etc.) for API endpoints
- Use `#[inject]` for dependency injection
- Clean up ALL test artifacts

### NEVER DO
- Use `mod.rs` files
- Commit without user instruction
- Save files to project directory (use `/tmp`)
- Leave skeleton tests
- Use glob imports (`use module::*`)

### Reinhardt Documentation Reference
- **Quick Start**: `@docs/repos/reinhardt-web/docs/GETTING_STARTED.md`
- **REST API Tutorial**: `@docs/repos/reinhardt-web/docs/tutorials/en/rest/`
- **Serialization**: `@docs/repos/reinhardt-web/docs/tutorials/en/rest/1-serialization.md`
- **Views**: `@docs/repos/reinhardt-web/docs/tutorials/en/rest/3-class-based-views.md`
- **ViewSets & Routers**: `@docs/repos/reinhardt-web/docs/tutorials/en/rest/6-viewsets-and-routers.md`
