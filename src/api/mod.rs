use axum::{
    http::{
        HeaderName, HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, DNT, REFERER, USER_AGENT},
        request::Parts,
    },
    routing::Router,
};

mod image_cutout;

pub fn api() -> Router {
    Router::new()
        .nest("/image-cutout", image_cutout::api())
        .layer(build_cors_layer())
}

/// 构建 CORS 层。
fn build_cors_layer() -> tower_http::cors::CorsLayer {
    tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::predicate(
            |origin: &HeaderValue, _request_parts: &Parts| {
                let origin = origin.to_str();
                origin
                    .is_ok_and(|origin| origin.contains("cinea.cc") || origin.contains("localhost"))
            },
        ))
        // tower_http的实现有问题，使用allow_credentials时必须手动指定allow methods和headers
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
            Method::PATCH,
        ])
        .allow_headers([
            AUTHORIZATION,
            CONTENT_TYPE,
            ACCEPT,
            DNT,
            REFERER,
            USER_AGENT,
            HeaderName::from_static("x-real-ip"),
            HeaderName::from_static("x-forwarded-for"),
            HeaderName::from_static("x-forwarded-proto"),
            HeaderName::from_static("x-forwarded-port"),
            HeaderName::from_static("x-request-id"),
            HeaderName::from_static("x-client-name"),
        ])
        .allow_credentials(true)
}
