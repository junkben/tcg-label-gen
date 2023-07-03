use crate::scryfall::ScryfallSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    name:     String,
    code:     String,
    date:     Option<String>,
    icon_url: String
}

impl From<ScryfallSet> for Label {
    fn from(value: ScryfallSet) -> Self {
        Label {
            name:     value.name().clone(),
            code:     value.code().clone(),
            icon_url: value.icon_svg_uri().clone(),
            date:     value.released_at().clone()
        }
    }
}
