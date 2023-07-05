#[derive(Debug, Clone, Serialize)]
pub enum CssFont {
    #[serde(rename = "Arial")]
    Arial,
    #[serde(rename = "Verdana")]
    Verdana,
    #[serde(rename = "Tahoma")]
    Tahoma,
    #[serde(rename = "'Trebuchet MS'")]
    TrebuchetMS,
    #[serde(rename = "'Times New Roman'")]
    TimesNewRoman,
    #[serde(rename = "Georgia")]
    Georgia,
    #[serde(rename = "'Courier New'")]
    CourierNew,
    #[serde(rename = "'Brush Script MT'")]
    BrushScriptMT,
    #[serde(rename = "Impact")]
    Impact,
    #[serde(rename = "'EB Garamond'")]
    EBGaramond,
    #[serde(rename = "'Source Sans Pro'")]
    SourceSansPro
}

impl Default for CssFont {
    fn default() -> Self { CssFont::Arial }
}
