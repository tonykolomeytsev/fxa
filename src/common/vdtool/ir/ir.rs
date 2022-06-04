use std::io::{BufWriter, Write};

use minidom::Element;

use crate::common::vdtool::error::VectorDrawableError;
use crate::common::vdtool::ir::path::PathNode;
use crate::common::vdtool::ir::svg::SvgNode;
use crate::common::vdtool::vdtool::ToVectorDrawable;

#[derive(Debug)]
pub enum IrNode {
    Svg(SvgNode),
    Path(PathNode),
}

impl IrNode {
    pub fn from(element: &Element) -> Result<Self, VectorDrawableError> {
        Ok(IrNode::Svg(SvgNode::from(&element)?))
    }
}

impl ToVectorDrawable for IrNode {
    fn to_vector_drawable<W: Write>(&self, w: &mut BufWriter<W>) -> Result<(), std::io::Error> {
        match &self {
            IrNode::Svg(node) => node.to_vector_drawable(w),
            IrNode::Path(node) => node.to_vector_drawable(w),
        }
    }
}
