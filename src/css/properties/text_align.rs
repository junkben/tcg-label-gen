#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CssTextAlign {
    Center,
    Left,
    Right,
    Justify
}

impl Default for CssTextAlign {
    fn default() -> Self { CssTextAlign::Left }
}
