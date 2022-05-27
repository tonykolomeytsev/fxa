use crate::common::fileutils::remove_temp_dir;

use crate::feature_cleanup::renderer::{FeatureCleanupRenderer, View};

pub fn cleanup() {
    let mut renderer = FeatureCleanupRenderer::new();
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
