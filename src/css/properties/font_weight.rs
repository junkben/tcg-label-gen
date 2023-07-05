#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CssFontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    ExtraBlack
}

impl Default for CssFontWeight {
    fn default() -> Self { CssFontWeight::Normal }
}
