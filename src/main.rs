use tera::{Context, Tera};

use crate::label_generator::LabelGenerator;

#[macro_use] extern crate serde;
#[macro_use] extern crate derive_getters;
extern crate tera;

mod cutting_guide;
mod label;
mod label_generator;
mod label_render;
mod paper_size;
mod scryfall;

fn main() { go().expect("go failed") }

fn go() -> anyhow::Result<()> {
    let label_generator = LabelGenerator::default();
    let label_svg = label_generator.create_label_svg_render()?;

    // https://tera.netlify.app/docs
    let svg_tera = Tera::new("templates/**/*.svg")?;
    let context = Context::from_serialize(label_svg)?;
    let svg = svg_tera.render("mtg/label_page.svg", &context)?;

    println!("Svg: {svg:?}");
    Ok(())
}
