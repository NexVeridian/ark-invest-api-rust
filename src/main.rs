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
use lazy_static::lazy_static;
use std::{env, net::SocketAddr, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};

mod routes;

lazy_static! {
    static ref NGINX: bool = match env::var("NGINX") {
        Ok(val) => val
            .to_lowercase()
            .parse::<bool>()
            .expect("Env string NGINX must be bool"),
        Err(_) => true,
    };
}

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
    let error_handler = || {
        ServiceBuilder::new().layer(HandleErrorLayer::new(|err: BoxError| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled error: {}", err),
            )
        }))
    };

    let rate_limit_global = |req_per_sec: u64| {
        ServiceBuilder::new()
            .layer(error_handler())
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(req_per_sec, Duration::from_secs(1)))
    };

    let rate_limit_ip = || {
        let config = Box::new(
            GovernorConfigBuilder::default()
                .per_millisecond(500)
                .burst_size(25)
                .use_headers()
                .key_extractor(SmartIpKeyExtractor)
                .finish()
                .unwrap(),
        );

        ServiceBuilder::new()
            .layer(error_handler())
            .layer(GovernorLayer {
                config: Box::leak(config),
            })
    };

    let cors = || {
        ServiceBuilder::new().layer(
            CorsLayer::new()
                .allow_methods(Method::GET)
                .allow_origin(Any),
        )
    };

    let route_layer = {
        rate_limit_global(1_000)
            .layer(cors())
            .layer(CompressionLayer::new().zstd(true))
            .option_layer(if *NGINX { Some(rate_limit_ip()) } else { None })
    };

    let app = ApiRouter::new()
        .route("/", Redoc::new("/api.json").axum_route())
        .layer(CompressionLayer::new().zstd(true))
        .route("/api.json", get(serve_api))
        .layer(CompressionLayer::new().zstd(true))
        .api_route(
            "/ark_holdings",
            get_with(routes::ark_holdings, |mut o| {
                o = o.id("ARK Holdings").description(
                    r"
| date | ticker | cusip | company | market_value | shares | share_price | weight |
|------|--------|-------|---------|--------------|--------|-------------|--------|
| date | str    | str   | str     | i64          | i64    | f64         | f64    |

### Example
`/ark_holdings?ticker=ARKK&start=2023-10-01&end=2023-11-01`",
                );
                description_date(o)
            }),
        )
        .layer(route_layer);

    let mut api = OpenApi {
        info: Info {
            summary: Some(
                "A REST API for ARK Invest holdings data, writen in rust using [axum](https://github.com/tokio-rs/axum), 
                Redoc/Swagger through [Aide](https://github.com/tamasfe/aide), 
                and parquet using [polars](https://github.com/pola-rs/polars)\n\nNot affiliated with Ark Invest
                ".to_owned(),
            ),
            description: Some(
                "[Github](https://github.com/NexVeridian/ark-invest-api-rust)\n\n[Contact Info](https://NexVeridian.com/about)".to_owned(),
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
                .layer(CompressionLayer::new().zstd(true))
                .into_make_service(),
        )
        .await
        .unwrap();
}
