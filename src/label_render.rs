use crate::{
    cutting_guide::CuttingGuide, label::plastic_divider::LabelPlasticDivider
};

#[derive(Debug, Serialize)]
pub struct MtgLabelSvgRender {
    pub labels:            Vec<LabelPlasticDivider>,
    pub horizontal_guides: Vec<CuttingGuide>,
    pub vertical_guides:   Vec<CuttingGuide>,
    pub paper_width:       u32,
    pub paper_height:      u32
}
