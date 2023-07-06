use crate::css::properties::*;

#[derive(Serialize, Debug, Clone, Builder, Default)]
#[builder(default)]
pub struct CssTextElement {
    x:                 CssLength,
    y:                 CssLength,
    font_size:         CssFontSize,
    font_family:       CssFont,
    font_weight:       CssFontWeight,
    dominant_baseline: CssDominantBaseline,
    text_align:        CssTextAlign,
    text_transform:    CssTextTransform,
    text:              String
}

impl CssTextElement {
    pub fn builder() -> CssTextElementBuilder {
        CssTextElementBuilder::default()
    }
}
