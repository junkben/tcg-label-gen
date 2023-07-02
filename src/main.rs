use crate::scryfall::{ScryfallList, ScryfallSet};

#[macro_use] extern crate serde;

mod scryfall;

fn main() -> anyhow::Result<()> {
    let resp = reqwest::blocking::get("https://api.scryfall.com/sets/")?;
    let resp_text = resp.text()?;
    let set_list: ScryfallList<ScryfallSet> =
        serde_json::from_str(&resp_text).unwrap();

    println!("{:?}", set_list);
    Ok(())
}
