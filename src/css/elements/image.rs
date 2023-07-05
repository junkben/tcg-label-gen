use crate::css::properties::*;

#[derive(Serialize, Debug, Clone, Builder, Default)]
#[builder(default)]
pub struct CssImageElement {
    x:      CssLength,
    y:      CssLength,
    width:  CssLength,
    height: CssLength,
    href:   String
}
