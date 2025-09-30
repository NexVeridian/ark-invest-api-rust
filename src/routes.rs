use aide::axum::IntoApiResponse;
use axum::extract::Query;

mod polars_utils;

pub async fn ark_holdings(
    ticker: Query<polars_utils::Ticker>,
    date_range: Query<polars_utils::DateRange>,
) -> impl IntoApiResponse {
    let df = polars_utils::get_parquet(&ticker.ticker.to_string()).unwrap();

    let filter_df = polars_utils::filter_date_range(df, &date_range).unwrap();

    axum::Json(polars_utils::to_json(filter_df).unwrap())
}
