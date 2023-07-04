#[derive(Debug, Clone)]
pub struct CssLength {
    value: u32,
    unit:  CssLengthUnit
}

impl serde::ser::Serialize for CssLength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        format!("{}{}", self.value, self.unit).serialize(serializer)
    }
}

impl CssLength {
    pub fn pixels(value: u32) -> CssLength {
        CssLength {
            value,
            unit: CssLengthUnit::Pixel
        }
    }

    pub fn centimeters(value: u32) -> CssLength {
        CssLength {
            value,
            unit: CssLengthUnit::Centimeter
        }
    }

    pub fn millimeters(value: u32) -> CssLength {
        CssLength {
            value,
            unit: CssLengthUnit::Millimeter
        }
    }

    pub fn quarter_millimeters(value: u32) -> CssLength {
        CssLength {
            value,
            unit: CssLengthUnit::QuarterMillimeter
        }
    }

    pub fn inches(value: u32) -> CssLength {
        CssLength {
            value,
            unit: CssLengthUnit::Inch
        }
    }

    pub fn picas(value: u32) -> CssLength {
        CssLength {
            value,
            unit: CssLengthUnit::Pica
        }
    }

    pub fn points(value: u32) -> CssLength {
        CssLength {
            value,
            unit: CssLengthUnit::Point
        }
    }
}

#[derive(Debug, Clone)]
pub enum CssLengthUnit {
    Pixel,
    Centimeter,
    Millimeter,
    QuarterMillimeter,
    Inch,
    Pica,
    Point
}

impl Default for CssLengthUnit {
    fn default() -> Self { CssLengthUnit::Pixel }
}

impl std::fmt::Display for CssLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CssLengthUnit::*;
        write!(f, "{}", match self {
            Pixel => "px",
            Centimeter => "cm",
            Millimeter => "mm",
            QuarterMillimeter => "Q",
            Inch => "in",
            Pica => "pc",
            Point => "pt"
        })
    }
}
