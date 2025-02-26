use crate::command::{create_user, delete_user};
use crate::gql::types::{CreateUserInput, UserObject}; // ✅ 修正
use crate::query::{get_user, list_users};
use async_graphql::{EmptySubscription, Object, Schema};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_user(&self, id: String) -> Option<UserObject> {
        let uuid = Uuid::parse_str(&id).ok()?; // ✅ `String` → `Uuid`
        get_user::get_user(uuid)
            .await
            .ok()
            .flatten()
            .map(UserObject::from)
    }

    // ユーザー一覧を取得するリゾルバを追加
    async fn users(&self) -> Vec<UserObject> {
        match list_users::list_users().await {
            Ok(users) => users.into_iter().map(UserObject::from).collect(),
            Err(e) => {
                println!("❌ ユーザー一覧取得エラー: {:?}", e);
                vec![] // エラー時は空の配列を返す
            }
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, input: CreateUserInput) -> async_graphql::Result<UserObject> {
        println!("📊 GraphQL create_user リゾルバが呼び出されました");

        match create_user::create_user(input.name, input.email).await {
            Ok(user_model) => {
                println!("✅ DB操作成功。ユーザーモデル: {:?}", user_model);
                println!("ID: {}", user_model.id);

                let user_object = UserObject {
                    id: user_model.id.to_string(),
                    name: user_model.name,
                    email: user_model.email,
                };

                println!("✅ 変換後の UserObject: {:?}", user_object);
                Ok(user_object)
            }
            Err(e) => {
                println!("❌ ユーザー作成エラー: {:?}", e);
                Err(async_graphql::Error::new(format!(
                    "ユーザーの作成に失敗しました: {:?}",
                    e
                )))
            }
        }
    }

    async fn delete_user(&self, id: String) -> async_graphql::Result<bool> {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(async_graphql::Error::new("Invalid UUID format")),
        };

        match delete_user::delete_user(uuid).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

pub fn create_schema(db: DatabaseConnection) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db) // `SeaORM` の `db` を context に追加
        .finish()
}
