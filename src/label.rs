use crate::scryfall::ScryfallSet;

#[derive(Serialize, Deserialize, Debug)]
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
        Label {
            name:     scryfall_set.name().clone(),
            code:     scryfall_set.code().clone(),
            icon_url: scryfall_set.icon_svg_uri().clone(),
            date:     scryfall_set.released_at().clone(),
            x:        position.0,
            y:        position.1
        }
    }
}
