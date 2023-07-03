use scryfall::get_all_sets;
use tera::Tera;

use crate::label::Label;

#[macro_use] extern crate serde;
#[macro_use] extern crate derive_getters;
extern crate tera;

mod label;
mod scryfall;

fn main() { go().unwrap() }

fn go() -> anyhow::Result<()> {
    let set_list = get_all_sets("json".to_owned(), true)?;
    let sets = set_list.data().clone();
    let labels = sets.into_iter().map(Label::from).collect::<Vec<_>>();

    let tera = Tera::new("templates/**/*.html")?;

    println!("Labels: {labels:?}");
    Ok(())
}
