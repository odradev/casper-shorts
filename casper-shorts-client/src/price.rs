use std::fs::File;

use reqwest::blocking::Client;
use serde_json::Value;
use thiserror::Error;

const HISTORICAL_DATA_FILE: &str = "casper-shorts-client/resources/historical_data.csv";
const CMC_URL: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
const CMC_API_KEY: &str = "COINMARKETCAP_API_KEY";

pub trait PriceProvider {
    fn get_cspr_price() -> Result<f64, PriceError>;
}

pub trait HistoricalPriceProvider {
    fn get_historical_cspr_price() -> Result<Vec<f64>, PriceError>;
}

pub struct CoinmarketcapProvider;

impl PriceProvider for CoinmarketcapProvider {
    fn get_cspr_price() -> Result<f64, PriceError> {
        let api_key = std::env::var(CMC_API_KEY).map_err(|_| PriceError::MissingAPIKey)?;
        let parameters = [("slug", "casper"), ("convert", "USD")];

        let client = Client::new();
        let response = client
            .get(CMC_URL)
            .headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Accepts", "application/json".parse().unwrap());
                headers.insert("X-CMC_PRO_API_KEY", api_key.parse().unwrap());
                headers
            })
            .query(&parameters)
            .send()?;

        let info: Value = response.json()?;
        let price = info["data"]["5899"]["quote"]["USD"]["price"]
            .as_f64()
            .ok_or(PriceError::InvalidPriceFormat)?;

        Ok(price)
    }
}

pub struct FilePriceProvider;

impl HistoricalPriceProvider for FilePriceProvider {
    fn get_historical_cspr_price() -> Result<Vec<f64>, PriceError> {
        let file = File::open(HISTORICAL_DATA_FILE)?;
        let mut rdr = csv::Reader::from_reader(file);

        let mut prices = vec![];
        for result in rdr.records() {
            let record = result.map_err(|_| PriceError::InvalidPriceFormat)?;
            let price = record
                .get(1)
                .unwrap_or_default()
                .parse::<f64>()
                .map_err(|_| PriceError::InvalidPriceFormat)?;
            prices.push(price);
        }
        Ok(prices)
    }
}

#[derive(Error, Debug)]
pub enum PriceError {
    #[error("data store disconnected")]
    ApiError(#[from] reqwest::Error),
    #[error("Invalid price format")]
    InvalidPriceFormat,
    #[error("Missing API key")]
    MissingAPIKey,
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}
