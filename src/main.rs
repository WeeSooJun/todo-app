use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
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
}
