use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use rust_cqrs::db::init_db;
use rust_cqrs::schema::{AppSchema, create_schema};

async fn graphql_hadler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ✅ `await` をつけて `DatabaseConnection` を取得
    let db = init_db().await.expect("Failed to initialize DB");

    // ✅ `db` を `create_schema()` に渡す
    let schema = create_schema(db);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_hadler))
            .route("/playground", web::get().to(graphql_playground))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
