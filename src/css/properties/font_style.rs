#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CssFontStyle {
    Normal,
    Italic,
    Oblique
}

impl Default for CssFontStyle {
    fn default() -> Self { CssFontStyle::Normal }
}
