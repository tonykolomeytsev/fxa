use std::fmt;
use std::fs;
use std::io::BufWriter;
use std::io::Write;

use crate::common::renderer::Renderer;
use crate::feature_config::view::View;

#[derive(Debug)]
pub struct ConfigFeatureError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for ConfigFeatureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}; {}", &self.message, &self.cause)
    }
}

pub fn create_default_config(path: &String) {
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path);

    let result = match file {
        Ok(file) => {
            let default_config_content = include_str!("../../../res/default_config.yaml");
            let mut writer = BufWriter::new(&file);
            writeln!(writer, "{}", default_config_content).or_else(|e| {
                let message = format!("Can't create config file {}", &path);
                let cause = format!("{}", e);
                Err(ConfigFeatureError { message, cause })
            })
        }
        Err(e) => {
            let message = format!("Can't create config file {}", &path);
            let cause = format!("{}", e);
            Err(ConfigFeatureError { message, cause })
        }
    };
    let renderer = Renderer();
    renderer.new_line();
    match result {
        Ok(()) => renderer.render(View::Done {
            message: format!("created config file {}", &path),
        }),
        Err(e) => renderer.render(View::Error {
            description: format!("{}", e),
        }),
    }
}
