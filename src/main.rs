use axum::{routing::get, Router};
use sqlx::{postgres::PgPoolOptions, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/todo-app")
        .await?;

    let rows = sqlx::query("SELECT * FROM valid_todo_state")
        .fetch_all(&pool)
        .await?;
    for row in rows.iter() {
        println!("{:?}", row.get::<String, &str>("valid_state"));
    }

    // build our application with a single route
    let app = Router::new().route(
        "/todos",
        get(|| async { "Todo 1 Todo 2" })
            .post(|| async { "Todo added!" })
            .patch(|| async { "Todo updated!" })
            .delete(|| async { "Todo deleted!" }),
    );

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
