use axum::extract::Query;
use chrono::NaiveDate;
use polars::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;

pub async fn get_parquet(file: String) -> Result<DataFrame, Box<dyn Error>> {
    let mut file = File::open(format!("data/parquet/{}.parquet", file))?;
    let df = ParquetReader::new(&mut file).finish()?;
    Ok(df)
}

pub async fn to_json(mut df: DataFrame) -> Result<Value, Box<dyn Error>> {
    let mut buffer = Vec::new();
    JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df)?;
    let json_string = String::from_utf8(buffer)?;
    let json: Value = serde_json::from_str(&json_string)?;
    Ok(json)
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DateRange {
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
}

pub async fn filter_date_range(
    df: DataFrame,
    date_range: Query<DateRange>,
) -> Result<DataFrame, Box<dyn Error>> {
    if date_range.start.or(date_range.end).is_none() {
        return Ok(df);
    }

    let mask = df["date"]
        .date()?
        .as_date_iter()
        .map(|x| {
            let date = x.unwrap();
            match (date_range.start, date_range.end) {
                (Some(start), Some(end)) => date >= start && date <= end,
                (Some(start), _) => date >= start,
                (_, Some(end)) => date <= end,
                _ => false,
            }
        })
        .collect();

    let filter_df = df.filter(&mask)?;
    Ok(filter_df)
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Serialize, Deserialize, JsonSchema, strum_macros::Display)]
pub enum TypeTicker {
    ARKVX,

    ARKF,
    ARKG,
    ARKK,
    ARKQ,
    ARKW,
    ARKX,

    ARKA,
    ARKZ,
    ARKC,
    ARKD,
    ARKY,
    ARKB,

    PRNT,
    IZRL,

    EUROPE_ARKI,
    EUROPE_ARKG,
    EUROPE_ARKK,

    CYBR,
    CYCL,
    FOOD,
    LIFE,
    LUSA,
    NFRA,
    PMNT,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Ticker {
    pub ticker: TypeTicker,
}
