use clap::{Parser, Subcommand};

/// Simple util to export resources from figma to android project
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    /// Figma personal access token, can be omitted if there is a env variable
    #[clap(short, long, env = "FIGMA_PERSONAL_TOKEN")]
    pub token: String,
    /// Subcommand: images, icons
    #[clap(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Export images/illustrations from Figma
    Images {
        /// Space separated images names
        names: Vec<String>,
        /// Path to yaml config. Use `fxn config` to generate default config here
        #[clap(short = 'c', long = "config")]
        path_to_config: String,
    },
    /// Generate default yaml config here
    Config {
        /// New config filename
        #[clap(default_value_t = String::from("fxn_default_config.yaml"))]
        path: String,
    },
}
