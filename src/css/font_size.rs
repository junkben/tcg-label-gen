use super::length::CssLength;

#[derive(Debug, Clone)]
pub enum CssFontSize {
    // Absolute Size Values
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
    XxxLarge,
    // Relative Size Values
    Smaller,
    Larger,
    // Length Values
    Length(CssLength),
    // Percentage Values
    Percentage(u32)
}

impl serde::ser::Serialize for CssFontSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        use CssFontSize::*;
        match self {
            XxSmall => "xx-small".serialize(serializer),
            XSmall => "x-small".serialize(serializer),
            Small => "small".serialize(serializer),
            Medium => "medium".serialize(serializer),
            Large => "large".serialize(serializer),
            XLarge => "x-large".serialize(serializer),
            XxLarge => "xx-large".serialize(serializer),
            XxxLarge => "xxx-large".serialize(serializer),
            Smaller => "smaller".serialize(serializer),
            Larger => "larger".serialize(serializer),
            Length(css_length) => css_length.serialize(serializer),
            Percentage(value) => format!("{value}%").serialize(serializer)
        }
    }
}
