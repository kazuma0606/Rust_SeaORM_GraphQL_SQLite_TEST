use crate::models::user::{Entity as User, Model};
use sea_orm::*;

pub async fn list_users() -> Result<Vec<Model>, DbErr> {
    let db = crate::db::get_db();
    let users = User::find().all(db).await?;
    Ok(users)
}
