use crate::{cutting_guide::CuttingGuide, label::Label};

#[derive(Debug, Serialize)]
pub struct MtgLabelSvgRender {
    pub labels:            Vec<Label>,
    pub horizontal_guides: Vec<CuttingGuide>,
    pub vertical_guides:   Vec<CuttingGuide>,
    pub paper_width:       u32,
    pub paper_height:      u32
}
