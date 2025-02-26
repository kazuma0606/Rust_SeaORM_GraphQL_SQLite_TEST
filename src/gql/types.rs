use crate::models::user::Model;
use async_graphql::{InputObject, SimpleObject}; // ✅ async_graphql の型をインポート

#[derive(SimpleObject, Debug)]
pub struct UserObject {
    #[graphql(name = "id")]
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<Model> for UserObject {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name,
            email: model.email,
        }
    }
}

// ✅ GraphQL の `createUser` の入力型を追加
#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}
