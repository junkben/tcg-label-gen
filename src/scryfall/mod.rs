mod errors;
mod lists;
mod sets;

pub use errors::*;
pub use lists::*;
use serde::{de, ser};
pub use sets::*;

pub trait ScryfallGetEndpoint {
    type Request: ser::Serialize;
    type Response: de::DeserializeOwned;

    fn url() -> String;
    fn get(request: Self::Request) -> anyhow::Result<Self::Response> {
        let url = Self::url();
        let client = reqwest::blocking::Client::new();
        let request_str = serde_json::to_string(&request)?;
        let raw_resp = client.get(url).body(request_str).send()?;
        let parsed_resp: Self::Response = raw_resp.json()?;
        Ok(parsed_resp)
    }
}
