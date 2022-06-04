use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use minidom::Element;

use crate::common::fileutils::TEMP_DIR_PATH;
use crate::common::vdtool::error::VectorDrawableError;

use crate::common::vdtool::ir::IrNode;

pub trait ToVectorDrawable {
    fn to_vector_drawable<W>(&self, w: &mut BufWriter<W>) -> Result<(), std::io::Error>
    where
        W: Write;
}

pub fn convert_svg_to_xml(file_path: &String) -> Result<String, VectorDrawableError> {
    let svg_content = fs::read_to_string(&file_path)
        .map_err(|e| VectorDrawableError::CannotReadSvg(file_path.clone(), e.to_string()))?;

    let svg_tree: Element = svg_content.parse().map_err(|e: minidom::Error| {
        VectorDrawableError::CannotParseSvg(file_path.clone(), e.to_string())
    })?;

    // Put xml-icon in the location of the original svg icon
    let original_icon_file_name = Path::new(file_path).file_stem().unwrap().to_str().unwrap();
    // Make full output path for webp-image.
    let xml_icon_path = format!("{}/{}.xml", TEMP_DIR_PATH, original_icon_file_name);
    // write to file
    let xml_file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(xml_icon_path)
        .unwrap();
    let mut writer = BufWriter::new(xml_file);
    let ir_node = IrNode::from(&svg_tree)?;
    ir_node.to_vector_drawable(&mut writer)?;

    // return ok
    Ok(file_path.clone())
}
