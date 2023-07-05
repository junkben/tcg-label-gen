use crate::{
    css::{
        elements::{
            CssImageElement, CssImageElementBuilder, CssTextElement,
            CssTextElementBuilder
        },
        properties::{CssFont, CssFontSize, CssFontWeight, CssLength}
    },
    scryfall::ScryfallSet
};

const NAME_LEN_MAX: usize = 24;

#[derive(Serialize, Debug, Clone, Default)]
pub struct Label {
    text_elements:  Vec<CssTextElement>,
    image_elements: Vec<CssImageElement>
}

impl Label {
    pub fn new(scryfall_set: ScryfallSet, x: u32, y: u32) -> Label {
        let name = match scryfall_set.name() {
            n if n.len() > NAME_LEN_MAX => n[..NAME_LEN_MAX].to_owned(),
            n => n.to_owned()
        };

        let name_text_element = CssTextElementBuilder::default()
            .x(CssLength::pixels(x + 30))
            .y(CssLength::pixels(y + 30))
            .font_size(CssFontSize::Length(CssLength::pixels(35)))
            .font_weight(CssFontWeight::Bold)
            .font_family(CssFont::EBGaramond)
            .text(name)
            .build()
            .unwrap();

        let set_text_element = CssTextElementBuilder::default()
            .x(CssLength::pixels(x + 30))
            .y(CssLength::pixels(y + 70))
            .font_size(CssFontSize::Length(CssLength::pixels(25)))
            .font_family(CssFont::SourceSansPro)
            .text(format!(
                "{} - {}",
                scryfall_set.code().clone().to_uppercase(),
                scryfall_set.released_at().clone().unwrap()
            ))
            .build()
            .unwrap();

        let set_icon = CssImageElementBuilder::default()
            .x(CssLength::pixels(x + 490))
            .y(CssLength::pixels(y + 20))
            .width(CssLength::pixels(70))
            .height(CssLength::pixels(70))
            .href(scryfall_set.icon_svg_uri().clone())
            .build()
            .unwrap();

        Label {
            text_elements:  Vec::from([name_text_element, set_text_element]),
            image_elements: Vec::from([set_icon])
        }
    }
}
