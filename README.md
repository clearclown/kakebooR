# kakebooR 🏠💰

Rust製の家計簿アプリケーション。[Reinhardt](https://github.com/kent8192/reinhardt-rs) Webフレームワークを使用。

## クイックスタート

```bash
# ビルド
cargo build

# サーバー起動（データベースは自動作成）
cargo run --bin runserver
```

サーバーが起動したら http://127.0.0.1:8000/ でアクセス可能です。

## 概要

kakebooRは、個人の収支管理を行うためのREST APIアプリケーションです。

- 📊 **収入・支出の記録** - CRUD操作で取引を管理
- 📁 **カテゴリ分類** - 収入/支出をカテゴリで整理
- 📈 **集計レポート** - 月次・年次・カテゴリ別の分析

## 技術スタック

| 項目 | 技術 |
|------|------|
| 言語 | Rust 2024 Edition |
| フレームワーク | Reinhardt (Django-like Web Framework) |
| データベース | SQLite（自動作成） |
| ORM | Reinhardt ORM |
| API | RESTful JSON API |

## API リファレンス

### Categories（カテゴリ）

| Method | Endpoint | 説明 |
|--------|----------|------|
| GET | `/api/categories/` | 一覧取得 |
| POST | `/api/categories/` | 新規作成 |
| GET | `/api/categories/{id}/` | 詳細取得 |
| PUT | `/api/categories/{id}/` | 更新 |
| DELETE | `/api/categories/{id}/` | 削除 |

**使用例:**

```bash
# カテゴリ作成
curl -X POST http://127.0.0.1:8000/api/categories/ \
  -H "Content-Type: application/json" \
  -d '{"name": "食費", "category_type": "expense"}'

# カテゴリ一覧
curl http://127.0.0.1:8000/api/categories/
```

### Transactions（収支記録）

| Method | Endpoint | 説明 |
|--------|----------|------|
| GET | `/api/transactions/` | 一覧取得 |
| POST | `/api/transactions/` | 新規作成 |
| GET | `/api/transactions/{id}/` | 詳細取得 |
| PUT | `/api/transactions/{id}/` | 更新 |
| DELETE | `/api/transactions/{id}/` | 削除 |

**使用例:**

```bash
# 取引作成（支出）
curl -X POST http://127.0.0.1:8000/api/transactions/ \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 1500,
    "category_id": 1,
    "description": "ランチ",
    "transaction_date": "2026-01-27T12:00:00Z",
    "transaction_type": "expense"
  }'

# 取引一覧
curl http://127.0.0.1:8000/api/transactions/
```

### Reports（集計）

| Method | Endpoint | 説明 |
|--------|----------|------|
| GET | `/api/reports/monthly/?year=YYYY&month=MM` | 月次サマリー |
| GET | `/api/reports/yearly/?year=YYYY` | 年次サマリー |
| GET | `/api/reports/by-category/` | カテゴリ別集計 |

**使用例:**

```bash
# 2026年1月の月次レポート
curl "http://127.0.0.1:8000/api/reports/monthly/?year=2026&month=1"

# 2026年の年次レポート
curl "http://127.0.0.1:8000/api/reports/yearly/?year=2026"

# カテゴリ別集計
curl http://127.0.0.1:8000/api/reports/by-category/
```

## データモデル

### Category

| フィールド | 型 | 説明 |
|-----------|-----|------|
| id | integer | 一意のID |
| name | string | カテゴリ名 |
| category_type | string | `income` または `expense` |
| icon | string? | アイコン識別子（オプション） |
| color | string? | カラーコード（オプション） |
| created_at | datetime | 作成日時 |

### Transaction

| フィールド | 型 | 説明 |
|-----------|-----|------|
| id | integer | 一意のID |
| amount | integer | 金額（円） |
| category_id | integer | カテゴリID |
| description | string | メモ・説明 |
| transaction_date | datetime | 取引日 |
| transaction_type | string | `income` または `expense` |
| created_at | datetime | 作成日時 |
| updated_at | datetime | 更新日時 |

## プロジェクト構成

```
kakebooR/
├── Cargo.toml
├── src/
│   ├── lib.rs              # ライブラリエントリポイント
│   ├── bin/
│   │   └── runserver.rs    # 開発サーバー
│   ├── config/
│   │   ├── settings.rs     # 設定
│   │   └── urls.rs         # URLルーティング
│   └── apps/
│       ├── categories/     # カテゴリ管理
│       ├── transactions/   # 収支記録
│       └── reports/        # 集計レポート
└── db.sqlite3              # SQLiteデータベース（自動生成）
```

## 開発コマンド

```bash
# ビルド
cargo build

# 開発サーバー起動
cargo run --bin runserver

# フォーマットチェック
cargo fmt --check

# Lintチェック
cargo clippy --all-features

# テスト実行
cargo test --all-features
```

## 前提条件

- Rust 1.91.1以上

## ライセンス

MIT License
