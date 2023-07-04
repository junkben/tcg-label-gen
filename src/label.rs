use crate::{
    css::{font_size::CssFontSize, length::CssLength},
    scryfall::ScryfallSet
};

const NAME_LEN_MAX: usize = 24;

#[derive(Serialize, Debug, Clone)]
pub struct Label {
    set_name: LabelTextElement,
    set_code: LabelTextElement,
    set_icon: LabelImageElement,
    x:        u32,
    y:        u32
}

impl Label {
    pub fn new(scryfall_set: ScryfallSet, position: (u32, u32)) -> Label {
        let name = match scryfall_set.name() {
            n if n.len() > NAME_LEN_MAX => n[..NAME_LEN_MAX].to_owned(),
            n => n.to_owned()
        };
        let set_name = LabelTextElement {
            offset_x:  30,
            offset_y:  30,
            font_size: CssFontSize::Length(CssLength::pixels(35)),
            text:      name
        };
        let set_code = LabelTextElement {
            offset_x:  30,
            offset_y:  70,
            font_size: CssFontSize::Length(CssLength::pixels(25)),
            text:      format!(
                "{} - {}",
                scryfall_set.code().clone(),
                scryfall_set.released_at().clone().unwrap()
            )
        };
        let set_icon = LabelImageElement {
            offset_x: 490,
            offset_y: 20,
            width:    CssLength::pixels(70),
            height:   CssLength::pixels(70),
            href:     scryfall_set.icon_svg_uri().clone()
        };
        Label {
            set_name,
            set_code,
            set_icon,
            x: position.0,
            y: position.1
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct LabelImageElement {
    offset_x: u32,
    offset_y: u32,
    width:    CssLength,
    height:   CssLength,
    href:     String
}

#[derive(Serialize, Debug, Clone)]
pub struct LabelTextElement {
    offset_x:  u32,
    offset_y:  u32,
    font_size: CssFontSize,
    text:      String
}
