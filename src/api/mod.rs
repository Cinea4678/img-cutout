use axum::routing::Router;

mod image_cutout;

pub fn api() -> Router {
    Router::new()
        .nest("/image-cutout", image_cutout::api())
}