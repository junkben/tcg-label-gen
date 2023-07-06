use crate::{
    css::{
        elements::{CssImageElement, CssTextElement},
        properties::{CssFont, CssFontSize, CssFontWeight, CssLength}
    },
    scryfall::ScryfallSet
};

const NAME_LEN_MAX: usize = 24;

#[derive(Serialize, Debug, Clone, Default)]
pub struct LabelPlasticDivider {
    text_elements:  Vec<CssTextElement>,
    image_elements: Vec<CssImageElement>
}

impl LabelPlasticDivider {
    pub fn new(
        scryfall_set: ScryfallSet,
        x: u32,
        y: u32
    ) -> LabelPlasticDivider {
        let name = match scryfall_set.name() {
            n if n.len() > NAME_LEN_MAX => n[..NAME_LEN_MAX].to_owned(),
            n => n.to_owned()
        };

        let name_text_element = CssTextElement::builder()
            .x(CssLength::pixels(x + 30))
            .y(CssLength::pixels(y + 30))
            .font_size(CssFontSize::Length(CssLength::pixels(35)))
            .font_weight(CssFontWeight::Bold)
            .font_family(CssFont::EBGaramond)
            .text(name)
            .build()
            .unwrap();

        let set_text_element = CssTextElement::builder()
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

        let set_icon = CssImageElement::builder()
            .x(CssLength::pixels(x + 490))
            .y(CssLength::pixels(y + 20))
            .width(CssLength::pixels(70))
            .height(CssLength::pixels(70))
            .href(scryfall_set.icon_svg_uri().clone())
            .build()
            .unwrap();

        LabelPlasticDivider {
            text_elements:  Vec::from([name_text_element, set_text_element]),
            image_elements: Vec::from([set_icon])
        }
    }
}
