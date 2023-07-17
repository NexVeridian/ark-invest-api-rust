use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
    redoc::Redoc,
    transform::TransformOperation,
};
use axum::{
    error_handling::HandleErrorLayer,
    http::{Method, StatusCode},
    BoxError, Extension, Json,
};
use std::{net::SocketAddr, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};

mod routes;

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

fn description_date(op: TransformOperation) -> TransformOperation {
    op.parameter_untyped("start", |p| {
        p.description("Start date range - Inclusive >= - ISO 8601")
    })
    .parameter_untyped("end", |p| {
        p.description("End date range - Inclusive <= - ISO 8601")
    })
}

#[tokio::main]
async fn main() {
    let rate_limit = |req_per_sec: u64| {
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(req_per_sec, Duration::from_secs(1)))
    };

    let cors = || {
        ServiceBuilder::new().layer(
            CorsLayer::new()
                .allow_methods(Method::GET)
                .allow_origin(Any),
        )
    };

    let app = ApiRouter::new()
        .route("/", Redoc::new("/api.json").axum_route())
        .api_route(
            "/arkvc_holdings",
            get_with(routes::arkvc_holdings, |mut o| {
                o = o.id("ARKVC Holdings");
                description_date(o)
            }),
        )
        .layer(rate_limit(5))
        .layer(cors())
        .api_route(
            "/ark_holdings",
            get_with(routes::ark_holdings, |mut o| {
                o = o.id("ARK* ETF Holdings");
                description_date(o)
            }),
        )
        .layer(rate_limit(20))
        .layer(cors())
        .route("/api.json", get(serve_api));

    let mut api = OpenApi {
        info: Info {
            summary: Some(
                "A REST API for ARK Invest holdings data, writen in rust using [axum](https://github.com/tokio-rs/axum), 
                Redoc/Swagger through [Aide](https://github.com/tamasfe/aide), 
                and parquet using [polars](https://github.com/pola-rs/polars)\n\nNot affiliated with Ark Invest
                ".to_owned(),
            ),
            description: Some(
                "[Github](https://github.com/NexVeridian/ark-invest-api-rust)\n\n[Contact Info](https://NexVeridian.com/About)".to_owned(),
            ),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(
            app.finish_api(&mut api)
                .layer(Extension(api))
                .into_make_service(),
        )
        .await
        .unwrap();
}
