use crate::models::user::Entity as User;
use sea_orm::*;
use uuid::Uuid;

pub async fn delete_user(id: Uuid) -> Result<(), DbErr> {
    let db = crate::db::get_db();

    let result = User::delete_by_id(id).exec(db).await?;

    if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound("User not found".to_string()));
    }

    Ok(())
}
