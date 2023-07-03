use crate::scryfall::ScryfallSet;

const NAME_LEN_MAX: usize = 24;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    name:     String,
    code:     String,
    date:     Option<String>,
    icon_url: String,
    x:        u32,
    y:        u32
}

impl Label {
    pub fn new(scryfall_set: ScryfallSet, position: (u32, u32)) -> Label {
        let name = match scryfall_set.name() {
            n if n.len() > NAME_LEN_MAX => n[..NAME_LEN_MAX].to_owned(),
            n => n.to_owned()
        };
        Label {
            name,
            code: scryfall_set.code().clone(),
            icon_url: scryfall_set.icon_svg_uri().clone(),
            date: scryfall_set.released_at().clone(),
            x: position.0,
            y: position.1
        }
    }
}
