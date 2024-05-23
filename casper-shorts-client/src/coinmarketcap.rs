use reqwest::blocking::Client;
use reqwest::Error;
use serde_json::Value;

pub fn get_cspr_price() -> Result<f64, Error> {
    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
    let api_key = std::env::var("COINMARKETCAP_API_KEY").unwrap();

    let parameters = [("slug", "casper"), ("convert", "USD")];

    let client = Client::new();
    let response = client
        .get(url)
        .headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Accepts", "application/json".parse().unwrap());
            headers.insert("X-CMC_PRO_API_KEY", api_key.parse().unwrap());
            headers
        })
        .query(&parameters)
        .send()?;

    let info: Value = response.json()?;
    let price = info["data"]["5899"]["quote"]["USD"]["price"].as_f64().unwrap();

    Ok(price)
}