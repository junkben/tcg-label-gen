use scryfall::get_all_sets;
use tera::Tera;

use crate::label::Label;

#[macro_use] extern crate serde;
#[macro_use] extern crate derive_getters;
extern crate tera;

mod label;
mod label_generator;
mod paper_size;
mod scryfall;

fn main() { go().unwrap() }

fn go() -> anyhow::Result<()> {
    let format = "json".to_owned();
    let pretty = true;
    let set_list = get_all_sets(format, pretty)?;
    let sets = set_list.data().clone();
    let labels = sets.into_iter().map(Label::from).collect::<Vec<_>>();

    // https://tera.netlify.app/docs
    let svg_tera = Tera::new("templates/**/*.svg")?;

    println!("Labels: {labels:?}");
    Ok(())
}
