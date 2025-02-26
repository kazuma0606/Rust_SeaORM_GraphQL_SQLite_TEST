# 🛠 技術スタックの検討

## 🚀 プロジェクト概要
本プロジェクトでは、Rust を用いて **CQRS + GraphQL API** を構築しました。  
`async-graphql` を採用し、`SeaORM` を使用した型安全なデータ操作を実現。  
開発時は **SQLite**、本番環境では **PostgreSQL** へ簡単に移行可能な構成になっています。

---

## 📌 技術スタック
| カテゴリ            | ツール / ライブラリ                           | 説明 |
|-----------------|---------------------------------|------------------------------------------------------------|
| **言語**        | Rust                           | 安全性・高速性を持つシステム言語 |
| **API**         | [async-graphql](https://github.com/async-graphql/async-graphql) | GraphQL の Rust 実装 |
| **Webフレームワーク** | [Actix-web](https://github.com/actix/actix-web) | 非同期対応の高性能な Web フレームワーク |
| **ORM (DB管理)** | [SeaORM](https://github.com/SeaQL/sea-orm) | Rust の非同期 ORM (SQLx ベース) |
| **データベース** | SQLite / PostgreSQL            | 開発時は SQLite、本番環境では PostgreSQL などを採用可能 |
| **ID管理**      | [uuid](https://github.com/uuid-rs/uuid) | ユニークな ID を生成 |
| **バリデーション** | [validator](https://github.com/Keats/validator) | フィールドのバリデーションを行う |
| **非同期処理**   | [tokio](https://github.com/tokio-rs/tokio) | Rust の非同期ランタイム |
| **依存管理**     | Cargo                           | Rust の標準パッケージマネージャ |
| **型管理**      | serde                           | JSON のシリアライズ/デシリアライズ |

---

## 📂 フォルダ構成
```
rust_cqrs/
│── Cargo.toml         # Rust のパッケージ設定
│── src/
│   ├── main.rs        # アプリケーションのエントリーポイント
│   ├── gql/           # GraphQL スキーマやリゾルバ
│   │   ├── schema.rs  # GraphQL スキーマ
│   │   ├── types.rs   # GraphQL のデータ構造
│   ├── db.rs          # データベースの初期化と管理
│   ├── models/        # データベースのモデル
│   │   ├── user.rs    # ユーザーモデル
│   ├── command/       # CQRS: コマンド (書き込み系処理)
│   │   ├── create_user.rs
│   │   ├── delete_user.rs
│   ├── query/         # CQRS: クエリ (読み込み系処理)
│   │   ├── get_user.rs
│   │   ├── list_users.rs
│── .gitignore         # Git 管理対象外の設定
│── README.md          # プロジェクト概要
```

---

## 🚀 セットアップと実行方法

### **🔹 1. Rust 環境のセットアップ**
Rust をインストールしていない場合は、以下のコマンドでセットアップしてください。
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### **🔹 2. リポジトリのクローン**
```sh
git clone https://github.com/your-repo/rust_cqrs.git
cd rust_cqrs
```

### **🔹 3. 依存関係のインストール**
```sh
cargo build
```

### **🔹 4. データベースの初期化**
```sh
cargo run --bin init_db
```

### **🔹 5. サーバーの起動**
```sh
cargo run
```

---

## 🛠 GraphQL API のテスト
GraphQL の API は、`http://localhost:8080/playground` にアクセスすると GraphQL Playground を使ってテストできます。

### **🔹 ユーザー作成 (Mutation)**
```graphql
mutation {
  createUser(input: { name: "Alice", email: "alice@example.com" }) {
    id
    name
    email
  }
}
```

### **🔹 ユーザー取得 (Query)**
```graphql
query {
  getUser(id: "7647b478-5848-49c7-b928-63cf7bda0384") {
    id
    name
    email
  }
}
```

### **🔹 ユーザー一覧取得 (Query)**
```graphql
query {
  users {
    id
    name
    email
  }
}
```

### **🔹 ユーザー削除 (Mutation)**
```graphql
mutation {
  deleteUser(id: "7647b478-5848-49c7-b928-63cf7bda0384")
}
```

---

## 🏗 今後の改善点
- **本番環境へのデプロイ**
  - Docker コンテナ化
  - PostgreSQL への移行
- **パフォーマンス最適化**
  - Redis などのキャッシュ導入
- **テストの追加**
  - `cargo test` を活用した自動テスト

---

## 📜 ライセンス
このプロジェクトは MIT ライセンスのもとで提供されます。

---

## 👨‍💻 開発者
- **Your Name** (GitHub: [yoshimura hisanori](https://github.com/kazuma0606))

---

🚀 **これで Rust での CQRS + GraphQL API の基盤が完成しました！** 🎉
```

