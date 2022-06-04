use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use usvg::Node;
use usvg::Options;
use usvg::Tree;

use crate::common::fileutils::TEMP_DIR_PATH;
use crate::common::vdtool::error::VectorDrawableError;

pub trait ToVectorDrawable {
    fn to_vector_drawable<W>(
        &self,
        w: &mut BufWriter<W>,
        node: Option<&Node>,
    ) -> Result<(), std::io::Error>
    where
        W: Write;
}

pub fn convert_svg_to_xml(file_path: &String) -> Result<String, VectorDrawableError> {
    let svg_content = fs::read_to_string(&file_path)
        .map_err(|e| VectorDrawableError::CannotReadSvg(file_path.clone(), e.to_string()))?;

    let svg_tree = Tree::from_str(&svg_content, &Options::default().to_ref())
        .map_err(|e| VectorDrawableError::CannotParseSvg(file_path.clone(), e.to_string()))?;

    // Put xml-icon in the location of the original svg icon
    let original_icon_file_name = Path::new(file_path).file_stem().unwrap().to_str().unwrap();
    // Make full output path for webp-image.
    let xml_icon_path = format!("{}/{}.xml", TEMP_DIR_PATH, original_icon_file_name);
    // write to file
    let xml_file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&xml_icon_path)
        .unwrap();
    let mut writer = BufWriter::new(xml_file);
    svg_tree.root().to_vector_drawable(&mut writer, None)?;

    // return ok
    Ok(xml_icon_path)
}

impl ToVectorDrawable for Node {
    fn to_vector_drawable<W>(
        &self,
        w: &mut BufWriter<W>,
        _: Option<&Node>,
    ) -> Result<(), std::io::Error>
    where
        W: Write,
    {
        match &*self.borrow() {
            usvg::NodeKind::Svg(svg) => svg.to_vector_drawable(w, Some(&self)),
            usvg::NodeKind::Path(path) => path.to_vector_drawable(w, Some(&self)),
            _ => Ok(()),
        }
    }
}
