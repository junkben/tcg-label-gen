#![allow(dead_code)]
use tera::{Context, Tera};

use crate::label_generator::LabelGenerator;

#[macro_use] extern crate serde;
#[macro_use] extern crate derive_getters;
extern crate tera;
#[macro_use] extern crate derive_builder;

mod css;
mod cutting_guide;
mod filters;
mod label;
mod label_generator;
mod label_render;
mod paper_size;
mod scryfall;

fn main() { go().expect("go failed") }

fn go() -> anyhow::Result<()> {
    let label_generator = LabelGenerator::default();
    let label_renders = label_generator.create_label_svg_renders()?;

    // https://tera.netlify.app/docs
    let svg_tera = Tera::new("templates/**/*.svg")?;
    let mut page = 1;
    for render in label_renders {
        println!("rendering labels_{:03}.html", page);
        let context = Context::from_serialize(render)?;
        let svg = svg_tera.render("mtg/label_page_proto.svg", &context)?;
        let filename = format!("output/labels_{:03}.html", page);
        std::fs::write(filename.as_str(), &svg)?;

        page += 1;
    }

    Ok(())
}
