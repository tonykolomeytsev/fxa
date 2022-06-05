use std::io::BufWriter;
use std::io::Write;

use usvg::PathSegment;
use usvg::{Color, FillRule, Paint, Path, PathData};

use crate::common::vdtool::vdtool::ToVectorDrawable;

impl ToVectorDrawable for Path {
    fn to_vector_drawable<W>(
        &self,
        w: &mut BufWriter<W>,
        _: Option<&usvg::Node>,
    ) -> Result<(), std::io::Error>
    where
        W: Write,
    {
        writeln!(w, "{:s$}<path", "", s = 4)?;

        // Add provided stroke params or default
        if let Some(stroke) = &self.stroke {
            // Add strokeWidth anyway
            write_stroke_width(w, stroke.width.value())?;

            // Add strokeColor anyway
            write_stroke_color(w, &stroke.paint)?;

            // Add strokeAlpha only if it differs from 1.0
            let stroke_alpha = stroke.opacity.value();
            if stroke_alpha != 1f64 {
                write_stroke_alpha(w, stroke_alpha)?;
            }

            // TODO: add strokeLineCap, strokeLineJoin, strokeMiterLimit
        } else {
            write_stroke_width(w, 1f64)?;
        }

        // Add provided fill params of default
        if let Some(fill) = &self.fill {
            // Add fillColor anyway
            write_fill_color(w, &fill.paint)?;

            // Add fillAlpha only if it differs from 1.0
            let fill_alpha = fill.opacity.value();
            if fill_alpha != 1f64 {
                write_fill_alpha(w, fill_alpha)?;
            }

            // Add fillType only if it differs from nonZero
            match fill.rule {
                FillRule::EvenOdd => write_fill_type(w, "evenOdd")?,
                _ => (),
            };
        } else {
            write_fill_color(w, &Paint::Color(Color::black()))?;
        }

        // Add pathData
        write_path_data(w, &self.data)?;

        // Close tag
        writeln!(w, " />\n")
    }
}

fn write_stroke_width<W: Write>(w: &mut BufWriter<W>, value: f64) -> Result<(), std::io::Error> {
    writeln!(w, "{:s$}android:strokeWidth=\"{:.}\"", "", value, s = 12)
}

fn write_stroke_alpha<W: Write>(w: &mut BufWriter<W>, value: f64) -> Result<(), std::io::Error> {
    writeln!(w, "{:s$}android:strokeAlpha=\"{:.}\"", "", value, s = 12)
}

fn rgb2hex(color: &Color) -> String {
    format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
}

fn write_stroke_color<W: Write>(w: &mut BufWriter<W>, value: &Paint) -> Result<(), std::io::Error> {
    let value = match value {
        Paint::Color(rgb_color) => rgb2hex(rgb_color),
        _ => "#ff000000".to_string(),
    };
    writeln!(w, "{:s$}android:strokeColor=\"{}\"", "", value, s = 12)
}

fn write_fill_color<W: Write>(w: &mut BufWriter<W>, value: &Paint) -> Result<(), std::io::Error> {
    let value = match value {
        Paint::Color(rgb_color) => rgb2hex(rgb_color),
        _ => "#ff000000".to_string(),
    };
    writeln!(w, "{:s$}android:fillColor=\"{}\"", "", value, s = 12)
}

fn write_fill_alpha<W: Write>(w: &mut BufWriter<W>, value: f64) -> Result<(), std::io::Error> {
    writeln!(w, "{:s$}android:fillAlpha=\"{:.}\"", "", value, s = 12)
}

fn write_fill_type<W: Write>(w: &mut BufWriter<W>, value: &str) -> Result<(), std::io::Error> {
    writeln!(w, "{:s$}android:fillType=\"{}\"", "", value, s = 12)
}

fn write_path_data<W: Write>(w: &mut BufWriter<W>, value: &PathData) -> Result<(), std::io::Error> {
    write!(w, "{:s$}android:pathData=\"", "", s = 12)?;
    for p in &value.0 {
        match p {
            PathSegment::ClosePath => write!(w, "Z")?,
            PathSegment::MoveTo { x, y } => write!(w, "M{:.},{:.}", x, y)?,
            PathSegment::LineTo { x, y } => write!(w, "L{:.},{:.}", x, y)?,
            PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => write!(w, "C{:.},{:.} {:.},{:.} {:.},{:.}", x1, y1, x2, y2, x, y)?,
        }
    }
    write!(w, "\"")
}
