use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE},
};
use tower_http::cors::CorsLayer;

pub fn cors(frontend_address: String) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(frontend_address.parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT, COOKIE])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
            Method::CONNECT,
            Method::HEAD,
            Method::TRACE,
        ])
}
