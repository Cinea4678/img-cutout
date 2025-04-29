use std::str::FromStr;

use axum::{
    Router,
    body::Body,
    extract::{Multipart, Query},
    http::{Response, StatusCode},
    routing::post,
};
use mime::{IMAGE_JPEG, IMAGE_PNG, Mime};
use serde::Deserialize;

use crate::service::image_cutout::image_cutout_static_white;

pub fn api() -> Router {
    Router::new().route("/", post(image_cutout))
}

#[derive(Debug, Clone, Deserialize)]
struct ImageCutoutParams {
    #[serde(default)]
    tolerance: Option<u8>,
}

async fn image_cutout(
    Query(params): Query<ImageCutoutParams>,
    mut multipart: Multipart,
) -> Result<Response<Body>, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap().to_string();
        let content_type = Mime::from_str(field.content_type().unwrap()).unwrap();
        let data = field.bytes().await.unwrap();

        log::info!(
            "Receiving: filename: {filename}, content_type: {:?}, data_len: {}",
            content_type,
            data.len()
        );

        if content_type != IMAGE_PNG && content_type != IMAGE_JPEG {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Unsupported content_type: {content_type}"),
            ));
        }

        match image::guess_format(&data) {
            Ok(format) => {
                if format != image::ImageFormat::Jpeg && format != image::ImageFormat::Png {
                    return Err((
                        axum::http::StatusCode::BAD_REQUEST,
                        "Invalid image format content".to_string(),
                    ));
                }

                let cutout =
                    image_cutout_static_white(&data, params.tolerance.unwrap_or(30)).unwrap();
                return Ok(Response::builder()
                    .header("Content-Type", IMAGE_PNG.to_string())
                    .body(Body::from(cutout))
                    .unwrap());
            }
            Err(_) => {
                return Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    "Failed to parse image content".to_string(),
                ));
            }
        }
    }

    Err((
        axum::http::StatusCode::BAD_REQUEST,
        "At least upload one image".to_string(),
    ))
}
