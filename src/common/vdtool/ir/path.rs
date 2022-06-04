use std::io::Write;
use std::{collections::BTreeMap, io::BufWriter};

use minidom::Element;

use crate::common::vdtool::pathparser::parse_path;
use crate::common::vdtool::res::SVG_D;
use crate::common::vdtool::{
    error::VectorDrawableError,
    res::{PRESENTATION_MAP, SVG_CLIP_RULE, SVG_FILL, SVG_FILL_RULE, SVG_STROKE, SVG_STROKE_WIDTH},
    svgcolor::color_svg2vd,
    vdtool::ToVectorDrawable,
};

#[derive(Debug)]
pub struct PathNode {
    attributes: BTreeMap<String, String>,
    path_data: Vec<PathDataNode>,
}

#[derive(Debug)]
pub struct PathDataNode(pub char, pub Vec<f32>);

impl PathNode {
    pub fn from(element: &Element) -> Result<Self, VectorDrawableError> {
        Ok(Self {
            attributes: attributes(&element)?,
            path_data: path_data(&element)?,
        })
    }
}

fn attributes(element: &Element) -> Result<BTreeMap<String, String>, VectorDrawableError> {
    let mut output = BTreeMap::new();
    for (name, value) in element.attrs() {
        if PRESENTATION_MAP.contains_key(name) {
            let value = match name {
                SVG_FILL_RULE | SVG_CLIP_RULE => match value {
                    "nonzero" => "nonZero",
                    "evenodd" => "evenOdd",
                    _ => value,
                },
                _ => value,
            };
            if value.starts_with("url(") {
                println!("Unsupported URL value in tag <{}>", element.name());
                continue;
            }
            if name == SVG_STROKE_WIDTH && value == "0" {
                output.remove(SVG_STROKE);
            }
            output.insert(name.to_string(), value.to_string());
        }
        // TODO: transform tag from SvdNode constructor()
    }
    Ok(output)
}

fn path_data(element: &Element) -> Result<Vec<PathDataNode>, VectorDrawableError> {
    match element.attr(SVG_D) {
        Some(d) => Ok(parse_path(d)?),
        None => Ok(vec![]),
    }
}

impl ToVectorDrawable for PathNode {
    fn to_vector_drawable<W>(&self, w: &mut BufWriter<W>) -> Result<(), std::io::Error>
    where
        W: Write,
    {
        // First, decide whether we can skip this path, since it has no visible effect.
        if self.path_data.is_empty() {
            return Ok(());
        }

        let (fill_color, empty_fill) = match self.attributes.get(SVG_FILL).map(|s| s.as_str()) {
            Some("none") | Some("#00000000") | None => ("#ff000000", true),
            Some(fill_color) => (fill_color, false),
        };
        let empty_stroke = match self.attributes.get(SVG_FILL).map(|s| s.as_str()) {
            Some("none") | Some("#00000000") | None => true,
            Some(_) => false,
        };
        if empty_fill && empty_stroke {
            return Ok(());
        }

        // Second, write the color info handling the default values.
        writeln!(w, "{:s$}<path", "", s = 4)?;
        let indent = format!("{:s$}", "", s = 12);
        if empty_fill {
            write!(w, "{}", indent)?;
            writeln!(w, "android:fillColor=\"{}\"", fill_color)?;
        }
        if !empty_stroke && !self.attributes.contains_key(SVG_STROKE_WIDTH) {
            write!(w, "{}", indent)?;
            writeln!(w, "android:strokeWidth=\"1\"")?;
        }

        // Last, write the path data and all associated attributes.
        write!(w, "{}", indent)?;
        write!(w, "android:pathData=\"")?;
        write_path_data(&self.path_data, w)?;
        write!(w, "\"")?;
        write_attribute_values(&self.attributes, w)?;
        writeln!(w, " />\n")?;

        Ok(())
    }
}

fn write_attribute_values<W: Write>(
    attributes: &BTreeMap<String, String>,
    w: &mut BufWriter<W>,
) -> Result<(), std::io::Error> {
    for (name, value) in attributes {
        // Get android attribute corresponding to svg attribute
        let android_attribute = match PRESENTATION_MAP.get(name.as_str()) {
            Some(&android_attribute) => android_attribute,
            None => continue,
        };

        let svg_value = value.trim();
        let vd_value = match color_svg2vd(svg_value) {
            Some(vd_value) => vd_value,
            None => {
                // <- TODO: gradient node
                match svg_value {
                    v if v.ends_with("px") => &v[..v.len() - 2],
                    v => v,
                }
            }
        };

        writeln!(w)?;
        write!(w, "{:s$}{}=\"{}\"", "", android_attribute, vd_value, s = 12)?;
    }
    Ok(())
}

fn write_path_data<W: Write>(
    d: &Vec<PathDataNode>,
    w: &mut BufWriter<W>,
) -> Result<(), std::io::Error> {
    for node in d {
        write!(w, "{}", node.0)?;

        let len = node.1.len();
        let mut implicit_line_to = false;
        let mut line_to_type = ' ';
        if (node.0 == 'm' || node.0 == 'M') && len > 2 {
            implicit_line_to = true;
            line_to_type = if node.0 == 'm' { 'l' } else { 'L' };
        }

        for j in 0..len {
            if j > 0 {
                write!(w, "{}", if j % 2 != 0 { ',' } else { ' ' })?;
            }
            if implicit_line_to && j == 2 {
                write!(w, "{}", line_to_type)?;
            }
            let param = node.1.iter().nth(j).unwrap();
            if param.is_infinite() {
                panic!("Invalid number: {}", param);
            }
            write!(w, "{:.}", param)?;
        }
    }
    Ok(())
}
