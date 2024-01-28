use crate::data_fetcher;
use crate::european_options;
use crate::european_options::Option;
use axum::response::IntoResponse;
use axum::{extract::Path, routing::get, Json, Router};
use axum_macros::debug_handler;
use serde::Serialize;
use std::error::Error;

pub fn router() -> Router {
    Router::new()
        .route("/get_best_matches/:symbol", get(get_best_matches))
        .route(
            "/get_last_year_prices/:symbol",
            get(get_prices_of_last_year),
        )
}

#[debug_handler]
async fn get_best_matches(Path(symbol): Path<String>) -> Result<Json<Vec<String>>, String> {
    let df = data_fetcher::new();
    let data = df.get_best_matches(&symbol).await;

    match data {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err("couldn't get best matches".to_string()),
    }
}

#[debug_handler]
async fn get_prices_of_last_year(
    Path(symbol): Path<String>,
) -> Result<Json<LastYearPrices>, String> {
    let prices: LastYearPrices;
    let df = data_fetcher::new();
    let data = df.get_prices_of_last_year(&symbol).await;
    match data {
        Ok((months, values)) => Ok(Json(LastYearPrices {
            months: months,
            prices: values,
        })),
        Err(_) => Err("couldn't get best last year prices".to_string()),
    }
}

#[derive(Serialize)]
struct LastYearPrices {
    months: Vec<u32>,
    prices: Vec<String>,
}
