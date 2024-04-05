use crate::routes::create_routes;
mod routes;
pub async fn app() {
    let app = create_routes();

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("->> App is running on port http://localhost:3000/...");
    axum::serve(listner, app).await.unwrap();
}
