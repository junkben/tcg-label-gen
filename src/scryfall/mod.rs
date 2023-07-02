mod errors;
mod lists;
mod sets;

pub use errors::*;
pub use lists::*;
pub use sets::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "object")]
pub enum ScryfallObject<T> {
    Set(ScryfallSet),
    List(ScryfallList<T>),
    Error(ScryfallError)
}
