use minidom::{Element, NSChoice};

use crate::common::vdtool::error::Svg2VectorError;

use crate::common::vdtool::vdtree::VdTree;

pub fn convert_svg_to_xml(file_path: &String) -> Result<String, Svg2VectorError> {
    let svg_content = std::fs::read_to_string(&file_path)
        .map_err(|e| Svg2VectorError::CannotReadSvg(file_path.clone(), e.to_string()))?;

    let svg_tree: Element = svg_content.parse().map_err(|e: minidom::Error| {
        Svg2VectorError::CannotParseSvg(file_path.clone(), e.to_string())
    })?;

    let vd_tree = parse(file_path, svg_tree)?;

    // write to file
    //...

    // return ok
    Ok(file_path.clone())
}

const TAG_SVG: &str = "svg";

fn parse(file_path: &String, svg_node: Element) -> Result<VdTree, Svg2VectorError> {
    // Check svg tree is a valid svg file with <svg> root tag
    if !svg_node.is(TAG_SVG, NSChoice::Any) {
        return Err(Svg2VectorError::InvalidContentNotRoot(file_path.clone()));
    }

    // Load dimensions of svg file and create node instance
    let vd_tree = VdTree::from(svg_node)?;

    unimplemented!()
}
