use crate::common::fileutils::remove_temp_dir;

use crate::common::renderer::Renderer;
use crate::feature_cleanup::view::View;

pub fn cleanup() {
    let renderer = Renderer();
    renderer.new_line();
    match remove_temp_dir() {
        Ok(()) => renderer.render(View::Done {
            message: String::new(),
        }),
        Err(e) => renderer.render(View::Error {
            description: format!("while deleting `.fxn` directory: {}", &e),
        }),
    }
}
