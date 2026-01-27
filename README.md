# kakebooR

Rust製の家計簿アプリケーション。[Reinhardt](https://github.com/kent8192/reinhardt-rs) Webフレームワークを使用。

## 概要

kakebooRは、個人の収支管理を行うためのWebアプリケーションです。REST APIベースで構築され、収入・支出の記録、カテゴリ分類、集計機能を提供します。

## 技術スタック

- **言語**: Rust 2024 Edition
- **フレームワーク**: Reinhardt (Django-like Web Framework for Rust)
- **データベース**: PostgreSQL
- **API**: RESTful API

## 機能

### 基本機能

- 収入・支出の記録（CRUD）
- カテゴリによる分類
- 日付・金額・メモの管理

### 集計機能

- 月次・年次の収支サマリー
- カテゴリ別の支出分析
- 予算との比較

## プロジェクト構成

```
kakebooR/
├── Cargo.toml
├── settings/
│   ├── base.toml
│   ├── local.toml
│   └── production.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── settings.rs
│   │   ├── urls.rs
│   │   └── apps.rs
│   ├── bin/
│   │   ├── runserver.rs
│   │   └── manage.rs
│   └── apps/
│       ├── transactions/    # 収支記録
│       │   ├── models.rs
│       │   ├── views.rs
│       │   ├── serializers.rs
│       │   └── urls.rs
│       ├── categories/      # カテゴリ管理
│       │   ├── models.rs
│       │   ├── views.rs
│       │   ├── serializers.rs
│       │   └── urls.rs
│       └── reports/         # 集計・レポート
│           ├── views.rs
│           └── urls.rs
└── docs/
    └── repos/
        └── reinhardt-web/   # Reinhardtドキュメント
```

## セットアップ

### 前提条件

- Rust 1.91.1以上
- PostgreSQL
- Docker（テスト用）

### インストール

```bash
# Reinhardt CLIのインストール
cargo install reinhardt-admin-cli

# 依存関係のインストール
cargo build

# 開発サーバーの起動
cargo run --bin runserver
```

### データベース設定

```bash
# マイグレーションの作成
cargo run --bin manage makemigrations

# マイグレーションの実行
cargo run --bin manage migrate
```

## API エンドポイント

### Transactions（収支記録）

| Method | Endpoint | 説明 |
|--------|----------|------|
| GET | `/api/transactions/` | 一覧取得 |
| POST | `/api/transactions/` | 新規作成 |
| GET | `/api/transactions/{id}/` | 詳細取得 |
| PUT | `/api/transactions/{id}/` | 更新 |
| DELETE | `/api/transactions/{id}/` | 削除 |

### Categories（カテゴリ）

| Method | Endpoint | 説明 |
|--------|----------|------|
| GET | `/api/categories/` | 一覧取得 |
| POST | `/api/categories/` | 新規作成 |
| GET | `/api/categories/{id}/` | 詳細取得 |
| PUT | `/api/categories/{id}/` | 更新 |
| DELETE | `/api/categories/{id}/` | 削除 |

### Reports（集計）

| Method | Endpoint | 説明 |
|--------|----------|------|
| GET | `/api/reports/monthly/` | 月次サマリー |
| GET | `/api/reports/yearly/` | 年次サマリー |
| GET | `/api/reports/by-category/` | カテゴリ別集計 |

## 開発

### コマンド

```bash
# フォーマットチェック
cargo fmt --check

# Lintチェック
cargo clippy --all-features

# テスト実行
cargo test --all-features

# 開発サーバー起動
cargo run --bin runserver
```

### 新しいアプリの作成

```bash
cargo run --bin manage startapp <app_name> --template-type restful
```

## ライセンス

MIT License
