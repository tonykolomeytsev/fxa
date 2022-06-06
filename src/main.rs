mod api;
mod common;
mod features;
mod models;

use clap::Parser;

use crate::features::cleanup as feature_cleanup;
use crate::features::config as feature_config;
use crate::features::icons as feature_icons;
use crate::features::images as feature_images;
use crate::models::entrypoint::{Args, Command};

fn main() {
    let args = Args::parse();
    match args.subcommand {
        Command::Config {
            new_config_filename,
        } => feature_config::create_default_config(&new_config_filename),
        Command::Images {
            token,
            path_to_config,
            names,
        } => feature_images::export_images(&token, &names, &path_to_config),
        Command::Icons {
            token,
            path_to_config,
            names,
        } => feature_icons::export_icons(&token, &names, &path_to_config),
        Command::Cleanup => feature_cleanup::cleanup(),
    }
}
