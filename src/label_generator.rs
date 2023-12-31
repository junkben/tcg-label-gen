use crate::{
    cutting_guide::CuttingGuide,
    filters::SetTypeFilter,
    label::plastic_divider::LabelPlasticDivider,
    label_render::MtgLabelSvgRender,
    paper_size::PaperSize,
    scryfall::{get_all_sets, ScryfallSet}
};

const PAPER_SIZE: PaperSize = PaperSize::Letter;
const COLUMNS: u32 = 3;
const ROWS: u32 = 20;
const MARGIN: u32 = 200;

pub struct LabelGenerator {
    paper_size:  PaperSize,
    num_columns: u32,
    num_rows:    u32,
    margin:      u32,
    set_filter:  SetTypeFilter
}

impl Default for LabelGenerator {
    fn default() -> Self {
        LabelGenerator {
            paper_size:  PAPER_SIZE,
            num_columns: COLUMNS,
            num_rows:    ROWS,
            margin:      MARGIN,
            set_filter:  SetTypeFilter::default()
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

    fn create_horizontal_cutting_guides(&self) -> Vec<CuttingGuide> {
        let row_height = self.row_height_mm();
        let mut horizontal_guides = Vec::new();

        let mut x1 = (self.margin as f32 / 2 as f32).floor() as u32;
        let mut x2 = (self.margin as f32 * 0.8 as f32).floor() as u32;
        for i in 0..=self.num_rows {
            horizontal_guides.push(CuttingGuide {
                x1,
                x2,
                y1: self.margin + i * row_height,
                y2: self.margin + i * row_height
            })
        }

        x1 = self.paper_size.width_mm() - x1;
        x2 = self.paper_size.width_mm() - x2;
        for i in 0..=self.num_rows {
            horizontal_guides.push(CuttingGuide {
                x1,
                x2,
                y1: self.margin + i * row_height,
                y2: self.margin + i * row_height
            })
        }

        horizontal_guides
    }

    fn create_vertical_cutting_guides(&self) -> Vec<CuttingGuide> {
        let column_width = self.column_width_mm();
        let mut vertical_guides = Vec::new();

        let mut y1 = (self.margin as f32 / 2 as f32).floor() as u32;
        let mut y2 = (self.margin as f32 * 0.8 as f32).floor() as u32;
        for i in 0..=self.num_columns {
            vertical_guides.push(CuttingGuide {
                x1: self.margin + i * column_width,
                x2: self.margin + i * column_width,
                y1,
                y2
            })
        }

        y1 = self.paper_size.height_mm() - y1;
        y2 = self.paper_size.height_mm() - y2;
        for i in 0..=self.num_columns {
            vertical_guides.push(CuttingGuide {
                x1: self.margin + i * column_width,
                x2: self.margin + i * column_width,
                y1,
                y2
            })
        }

        vertical_guides
    }

    fn get_sets(&self) -> anyhow::Result<Vec<ScryfallSet>> {
        let set_list = get_all_sets()?;
        let sets = set_list.data().clone();
        let filtered_sets = sets
            .into_iter()
            .filter(|set| self.set_filter.allowed(set.set_type()))
            .collect::<Vec<_>>();
        Ok(filtered_sets)
    }

    fn create_labels(&self) -> anyhow::Result<Vec<LabelPlasticDivider>> {
        let sets = self.get_sets()?;
        let (start_x, start_y) = (self.margin, self.margin);

        let (mut x, mut y) = (start_x, start_y);
        let mut labels = Vec::new();
        for scryfall_set in sets {
            labels.push(LabelPlasticDivider::new(scryfall_set, x, y));

            match labels.len() as u32 {
                // Start a new page if needed
                n if n % self.num_labels_per_page() == 0 => {
                    x = start_x;
                    y = start_y;
                },
                // Start a new column if needed
                n if n % self.num_rows == 0 => {
                    x += self.column_width_mm();
                    y = start_y;
                },
                // Adjust y for next row, x remains the same
                _ => {
                    y += self.row_height_mm();
                }
            }
        }

        Ok(labels)
    }

    pub fn create_label_svg_renders(
        &self
    ) -> anyhow::Result<Vec<MtgLabelSvgRender>> {
        let labels: Vec<LabelPlasticDivider> = self.create_labels()?;
        let horizontal_guides = self.create_horizontal_cutting_guides();
        let vertical_guides = self.create_vertical_cutting_guides();
        let paper_width = self.paper_size.width_mm();
        let paper_height = self.paper_size.height_mm();

        let renders = labels
            .chunks(self.num_labels_per_page() as usize)
            .into_iter()
            .map(|labels| MtgLabelSvgRender {
                labels: labels.to_vec(),
                horizontal_guides: horizontal_guides.clone(),
                vertical_guides: vertical_guides.clone(),
                paper_width,
                paper_height
            })
            .collect::<Vec<_>>();
        Ok(renders)
    }
}
