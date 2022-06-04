use super::res::COLOR_MAP;

pub fn color_svg2vd(color: &str) -> Option<&str> {
    let color = color.trim();

    if color.starts_with("#") {
        return Some(color);
    }
    if color == "none" {
        return Some("#00000000");
    }

    if color.starts_with("rgb(") && color.ends_with(")") {
        // todo
    }
    if color.starts_with("srgb(") && color.ends_with(")") {
        // todo
    }

    return COLOR_MAP.get(color.to_lowercase().as_str()).map(|&v| v);
}
