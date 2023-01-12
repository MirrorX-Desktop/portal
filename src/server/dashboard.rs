use crate::{handler, DASHBOARD_PORT};
use axum::{
    body::{boxed, Full},
    handler::HandlerWithoutStateExt,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use std::net::{Ipv4Addr, SocketAddr};

#[derive(RustEmbed)]
#[folder = "dashboard/build"]
struct DashboardDist;

pub async fn launch_dashboard_server() {
    let dashboard_listen_addr: SocketAddr = (Ipv4Addr::LOCALHOST, *DASHBOARD_PORT).into();

    let app = Router::new()
        .route("/", get(index_page_handler))
        .route("/overview",get(overview_page_handler))
        .route("/favicon.png", get(favicon_handler))
        .route("/robots.txt", get(robots_handler))
        .route("/vite-manifest.json", get(vite_manifest_handler))
        .route_service("/_app/*file", static_handler.into_service())
        .fallback_service(get(not_found));

    let app = app.route("/api/stat/details", get(handler::dashboard::stat::details));

    // Start listening on the given address.

    tracing::info!("http dashboard listening on {}", dashboard_listen_addr);
    let http_future = axum::Server::bind(&dashboard_listen_addr).serve(app.into_make_service());

    tokio::select! {
        _ = http_future => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    tracing::info!("http dashboard server exit");
    std::process::exit(1);
}

async fn index_page_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("index.html")).await
}

async fn overview_page_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("overview.html")).await
}

async fn favicon_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("favicon.png")).await
}

async fn robots_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("robots.txt")).await
}

async fn vite_manifest_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("vite-manifest.json")).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    tracing::info!("request path: {}", uri.path());

    let mut path = uri.path().trim_start_matches('/');

    if path.is_empty() {
        path = "index.html";
    }

    match DashboardDist::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .unwrap()
        }
        None => (StatusCode::NOT_FOUND, "Page not found").into_response(),
    }
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Page not found")
}
