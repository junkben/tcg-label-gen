#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CssTextTransform {
    Uppercase,
    Lowercase,
    Capitalize
}

impl Default for CssTextTransform {
    fn default() -> Self { CssTextTransform::Capitalize }
}
