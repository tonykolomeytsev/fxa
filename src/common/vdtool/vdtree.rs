use minidom::Element;

use super::error::Svg2VectorError;

pub struct VdTree {
    view_box: ViewBox,
    width: f32,
    height: f32,
    scale_factor: f32,
}

pub struct ViewBox(f32, f32, f32, f32);

struct Size {
    value: f32,
    measure: Measure,
}
enum Measure {
    PIXELS,
    PERCENTS,
}

impl VdTree {
    pub fn from(svg_node: Element) -> Result<Self, Svg2VectorError> {
        let (view_box, width, height) = parse_dimensions(svg_node)?;
        Ok(Self {
            view_box,
            width,
            height,
            scale_factor: 1f32,
        })
    }
}

/// Return dimensions of root `<svg>` tag
///
/// # Arguments
///
/// * `svg_node` - root `<svg>` tag of svg file
fn parse_dimensions(svg_node: Element) -> Result<(ViewBox, f32, f32), Svg2VectorError> {
    // extract svg dimensions like vdtool do
    let (view_box, width, height) = match (
        get_view_box(&svg_node),
        get_dimension(&svg_node, "width"),
        get_dimension(&svg_node, "height"),
    ) {
        (None, Some(width), Some(height)) => {
            let view_box = ViewBox(0f32, 0f32, width.value, height.value);
            (view_box, width, height)
        }
        (Some(view_box), None, None) => {
            let width = Size {
                value: view_box.2,
                measure: Measure::PIXELS,
            };
            let height = Size {
                value: view_box.3,
                measure: Measure::PIXELS,
            };
            (view_box, width, height)
        }
        _ => return Err(Svg2VectorError::InvalidDimensionSvgTag),
    };

    let pixel_width = match width.measure {
        Measure::PERCENTS => view_box.2 * width.value / 100f32,
        _ => width.value,
    };
    let pixel_height = match height.measure {
        Measure::PERCENTS => view_box.3 * height.value / 100f32,
        _ => height.value,
    };

    Ok((view_box, pixel_width, pixel_height))
}

/// Get `width` or `height` attribute value from root `<svg>` tag
///
/// # Arguments
///
/// * `svg_node` - root `<svg>` tag of svg file
fn get_dimension(svg_node: &Element, dimension: &str) -> Option<Size> {
    let attr_value = svg_node.attr(dimension)?;
    let is_percents = attr_value.chars().last() == Some('%');
    let value = attr_value
        .chars()
        .take_while(|&ch| ch.is_digit(10u32) || ch == '-')
        .collect::<String>()
        .parse::<f32>();
    match value {
        Ok(value) => Some(Size {
            value,
            measure: if is_percents {
                Measure::PERCENTS
            } else {
                Measure::PIXELS
            },
        }),
        Err(_) => None,
    }
}

/// Get `viewBox` attribute value from root `<svg>` tag
///
/// # Arguments
///
/// * `svg_node` - root `<svg>` tag of svg file
fn get_view_box(svg_node: &Element) -> Option<ViewBox> {
    let attr_value = svg_node.attr("viewBox")?;
    let sizes = attr_value
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
