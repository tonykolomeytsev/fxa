use crate::common::fileutils::remove_temp_dir;

use crate::common::renderer::Renderer;
use crate::feature_cleanup::view::View;

pub fn cleanup() {
    let renderer = Renderer();
    renderer.new_line();
    match remove_temp_dir() {
        Ok(()) => renderer.render(View::Done),
        Err(e) => renderer.render(View::Error(format!(
            "Can't delete temporary `.fxn` directory: {}",
            &e
        ))),
    }
}
