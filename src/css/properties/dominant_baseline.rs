#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CssDominantBaseline {
    Auto,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Central,
    Middle,
    TextAfterEdge,
    TextBeforeEdge,
    TextTop
}

impl Default for CssDominantBaseline {
    fn default() -> Self { CssDominantBaseline::Hanging }
}
