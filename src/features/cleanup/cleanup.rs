use crate::common::fileutils::remove_temp_dir;

pub fn cleanup() {
    remove_temp_dir().unwrap()
}
