use crate::paper_size::PaperSize;

const PAPER_SIZE: PaperSize = PaperSize::Letter;
const COLUMNS: u32 = 4;
const ROWS: u32 = 15;
const MARGIN: u32 = 200;

pub struct LabelGenerator {
    paper_size:  PaperSize,
    num_columns: u32,
    num_rows:    u32,
    margin:      u32
}

impl Default for LabelGenerator {
    fn default() -> Self {
        LabelGenerator {
            paper_size:  PAPER_SIZE,
            num_columns: COLUMNS,
            num_rows:    ROWS,
            margin:      MARGIN
        }
    }
}

impl LabelGenerator {
    pub fn effective_width_mm(&self) -> u32 {
        self.paper_size.width_mm() - (2 * self.margin)
    }

    pub fn effective_height_mm(&self) -> u32 {
        self.paper_size.height_mm() - (2 * self.margin)
    }

    pub fn column_width_mm(&self) -> u32 {
        self.effective_width_mm() / self.num_columns
    }

    pub fn row_height_mm(&self) -> u32 {
        self.effective_height_mm() / self.num_rows
    }

    pub fn num_labels_per_page(&self) -> u32 {
        self.num_columns * self.num_rows
    }
}
