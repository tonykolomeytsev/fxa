use minidom::Element;
use std::io::BufWriter;
use std::io::Write;

use crate::common::vdtool::error::VectorDrawableError;
use crate::common::vdtool::ir::ir::IrNode;
use crate::common::vdtool::ir::path::PathNode;
use crate::common::vdtool::res::UNSUPPORTED_SVG_NODES;
use crate::common::vdtool::vdtool::ToVectorDrawable;

#[derive(Debug)]
pub struct SvgNode {
    children: Vec<IrNode>,
    view_box: ViewBox,
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct ViewBox(pub f32, pub f32, pub f32, pub f32);

#[derive(Debug)]
struct Size(f32, Unit);

#[derive(Debug)]
enum Unit {
    PIXELS,
    PERCENTS,
}

impl SvgNode {
    pub fn from(element: &Element) -> Result<Self, VectorDrawableError> {
        let (view_box, width, height) =
            dimensions(element).ok_or(VectorDrawableError::InvalidDimensionSvgTag)?;
        Ok(Self {
            children: children(&element)?,
            view_box,
            width,
            height,
        })
    }
}

fn width(e: &Element) -> Option<Size> {
    parse_dim_by_name(e, "width")
}

fn height(e: &Element) -> Option<Size> {
    parse_dim_by_name(e, "height")
}

fn view_box(e: &Element) -> Option<ViewBox> {
    let sizes = e
        .attr("viewBox")?
        .split(' ')
        .filter_map(|s| s.parse::<f32>().map_or(None, Some))
        .collect::<Vec<f32>>();

    if sizes.len() != 4 {
        None
    } else {
        Some(ViewBox(
            *sizes.get(0)?,
            *sizes.get(1)?,
            *sizes.get(2)?,
            *sizes.get(3)?,
        ))
    }
}

fn dimensions(e: &Element) -> Option<(ViewBox, f32, f32)> {
    let (view_box, width, height) = match (view_box(e), width(e), height(e)) {
        (None, Some(width), Some(height)) => {
            let view_box = ViewBox(0f32, 0f32, width.0, height.0);
            (view_box, width, height)
        }
        (Some(view_box), None, None) => {
            let width = Size(view_box.2, Unit::PIXELS);
            let height = Size(view_box.3, Unit::PIXELS);
            (view_box, width, height)
        }
        (Some(view_box), Some(width), Some(height)) => (view_box, width, height),
        _ => return None,
    };

    let pixel_width = match width.1 {
        Unit::PERCENTS => view_box.2 * width.0 / 100f32,
        _ => width.0,
    };
    let pixel_height = match height.1 {
        Unit::PERCENTS => view_box.3 * height.0 / 100f32,
        _ => height.0,
    };

    Some((view_box, pixel_width, pixel_height))
}

fn parse_dim_by_name(e: &Element, name: &str) -> Option<Size> {
    let attr_value = e.attr(name)?;
    let measure = if attr_value.chars().last() == Some('%') {
        Unit::PERCENTS
    } else {
        Unit::PIXELS
    };
    let value = attr_value
        .chars()
        .take_while(|&ch| ch.is_digit(10u32))
        .collect::<String>()
        .parse::<f32>();
    match value {
        Ok(value) => Some(Size(value, measure)),
        Err(_) => None,
    }
}

fn children(element: &Element) -> Result<Vec<IrNode>, VectorDrawableError> {
    let mut output = Vec::new();
    for e in element.children() {
        if UNSUPPORTED_SVG_NODES.contains(e.name()) {
            println!("Unsupported tag {}", e.name());
            continue;
        }
        match e.name() {
            "path" => output.push(IrNode::Path(PathNode::from(e)?)),
            _ => println!("Unsupported by app tag {}", e.name()),
        }
    }
    Ok(output)
}

impl ToVectorDrawable for SvgNode {
    fn to_vector_drawable<W>(&self, w: &mut BufWriter<W>) -> Result<(), std::io::Error>
    where
        W: Write,
    {
        // Render header
        writeln!(
            w,
            "<vector xmlns:android=\"http://schemas.android.com/apk/res/android\"\n\
            \x20       android:width=\"{:.}dp\"\n\
            \x20       android:height=\"{:.}dp\"\n\
            \x20       android:viewportWidth=\"{:.}\"\n\
            \x20       android:viewportHeight=\"{:.}\">\n",
            self.width, self.height, self.view_box.2, self.view_box.3,
        )?;

        // Render content of every child
        for child in &self.children {
            child.to_vector_drawable(w)?;
        }

        // Render footer
        writeln!(w, "</vector>")?;
        Ok(())
    }
}
