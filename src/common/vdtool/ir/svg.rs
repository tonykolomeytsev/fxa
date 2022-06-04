use std::io::{BufWriter, Write};

use usvg::{Node, Svg};

use crate::common::vdtool::vdtool::ToVectorDrawable;

impl ToVectorDrawable for Svg {
    fn to_vector_drawable<W>(
        &self,
        w: &mut BufWriter<W>,
        node: Option<&Node>,
    ) -> Result<(), std::io::Error>
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
            self.size.width(),
            self.size.height(),
            self.view_box.rect.width(),
            self.view_box.rect.height(),
        )?;

        // Render content of every child
        for child in node.unwrap().children() {
            child.to_vector_drawable(w, None)?;
        }

        // Render footer
        writeln!(w, "</vector>")?;
        Ok(())
    }
}
