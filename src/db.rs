use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::OnceLock;

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

// ✅ `DatabaseConnection` を返すように変更
pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect("sqlite://users.db?mode=rwc").await?;
    DB.set(db.clone()).ok();
    Ok(db) // ✅ `DatabaseConnection` を返す
}

// ✅ 既存の `get_db()` もそのまま使える
pub fn get_db() -> &'static DatabaseConnection {
    DB.get().expect("Database not initialized")
}
