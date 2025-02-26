use crate::models::user::{Entity as User, Model};
use sea_orm::*;
use uuid::Uuid; // ✅ UUID を追加

pub async fn get_user(id: Uuid) -> Result<Option<Model>, DbErr> {
    let db = crate::db::get_db();
    let user = User::find_by_id(id).one(db).await?;
    Ok(user)
}
