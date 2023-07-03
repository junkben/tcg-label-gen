use scryfall::get_all_sets;

#[macro_use] extern crate serde;

mod scryfall;

fn main() { go().unwrap() }

fn go() -> anyhow::Result<()> {
    let sets = get_all_sets("json".to_owned(), true)?;
    println!("Sets: {sets:?}");
    Ok(())
}
