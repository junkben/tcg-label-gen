#[allow(dead_code)]
pub enum PaperSize {
    Letter,
    Legal,
    QuadrupleA0,
    DoubleA0,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10
}

impl Default for PaperSize {
    fn default() -> Self { PaperSize::Letter }
}

impl PaperSize {
    /// https://www.papersizes.org/a-paper-sizes.htm
    fn width_and_height_mm(&self) -> (u32, u32) {
        use PaperSize::*;
        match self {
            Letter => (2159, 2794),
            Legal => (2159, 3556),
            QuadrupleA0 => (16820, 23780),
            DoubleA0 => (11890, 16820),
            A0 => (8410, 11890),
            A1 => (5940, 8410),
            A2 => (4200, 5940),
            A3 => (2970, 4200),
            A4 => (2100, 2970),
            A5 => (1480, 2100),
            A6 => (1050, 1480),
            A7 => (740, 1050),
            A8 => (520, 740),
            A9 => (370, 520),
            A10 => (260, 370)
        }
    }

    pub fn width_mm(&self) -> u32 {
        let (width, _) = self.width_and_height_mm();
        width
    }

    pub fn height_mm(&self) -> u32 {
        let (_, height) = self.width_and_height_mm();
        height
    }
}
