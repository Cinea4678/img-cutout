pub mod api;
pub mod service;

#[tokio::main]
async fn main() {
    let app = api::api();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:11509").await.unwrap();    
    axum::serve(listener, app).await.unwrap();
}
