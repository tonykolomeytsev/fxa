use std::collections::HashMap;

use crate::{
    common::{
        gathering::view::View, renderer::Renderer, suffixes::SuffixExt,
        suggestions::generate_name_suggestions,
    },
    models::config::AppConfig,
};

pub struct InfoGeneratorEntry {
    pub user_name: String,
    pub figma_name: String,
    pub figma_id: String,
    pub scale_name: String,
    pub scale_value: f32,
    pub night: bool,
}

/// Checking the presence of all images in the frame by name.
///
/// There are three cases when checking:
/// 1. An image with the same name has been found.
///    In this case, add it to the download queue.
/// 2. An image with this name was not found, but images with a name with
///    different suffixes (such as `_light`, `_dark`, `_android`) were found.
///    We add such images to the upload queue.
/// 3. An image with that name was not found at all.
///    In this case, we warn the user and give them suggestions with similar image names.
///
/// # Arguments
///
/// - `format` - images format
/// - `names` - images, requested by users
/// - `names_to_ids` - map with names and ids of images available in frame
pub fn gather_names<G, I>(
    app_config: &AppConfig,
    frame_name: &String,
    names: &[String],
    names_to_ids: &HashMap<String, String>,
    single_scale_format: bool,
    info_generator: G,
) -> Vec<I>
where
    G: Fn(InfoGeneratorEntry) -> I,
{
    let renderer = Renderer();
    // Just for renderer and suggestions generator
    let available_names = names_to_ids
        .iter()
        .map(|(k, _)| k.clone())
        .collect::<Vec<String>>();
    let mut queue: Vec<I> = Vec::new();

    for user_name in names {
        // First, look for an images whose names match the names requested by the user
        if names_to_ids.contains_key(user_name) {
            // Notify user
            renderer.render(View::FoundSimple(user_name.clone()));
            // Add every desired scale to download queue
            for_each_scale(
                &app_config,
                single_scale_format,
                &mut |i| queue.push(i),
                |scale_name, scale_value| {
                    let e = InfoGeneratorEntry {
                        user_name: user_name.clone(),
                        figma_name: user_name.clone(),
                        figma_id: names_to_ids
                            .get(user_name)
                            .expect("Because map already contains key")
                            .clone(),
                        scale_name,
                        scale_value,
                        night: false,
                    };
                    info_generator(e)
                },
            );
            continue;
        }

        // Second, look for an images with suffixes in their names
        let names_with_theme_suffixes =
            Vec::from([user_name.with_light_suffix(), user_name.with_dark_suffix()]);
        if names_with_theme_suffixes
            .iter()
            .all(|n| available_names.contains(n))
        {
            // Notify user
            renderer.render(View::FoundThemed(user_name.clone()));
            // Add each name (with _light/_dark suffixes) to download queue
            for name in names_with_theme_suffixes {
                for_each_scale(
                    &app_config,
                    single_scale_format,
                    &mut |i| queue.push(i),
                    |scale_name, scale_value| {
                        let e = InfoGeneratorEntry {
                            user_name: user_name.clone(),
                            figma_name: name.clone(),
                            figma_id: names_to_ids
                                .get(&name)
                                .expect("Because map already contains key")
                                .clone(),
                            scale_name,
                            scale_value,
                            night: name.dark(),
                        };
                        info_generator(e)
                    },
                )
            }
            continue;
        }

        // TODO: Tell the user that we will not export only one configuration.

        // Third, notify user about resource with desired name is missing in the frame
        match generate_name_suggestions(&user_name, &available_names) {
            Some(suggestions) => renderer.render(View::NotFoundButSuggestions(
                user_name.clone(),
                frame_name.clone(),
                suggestions,
            )),
            None => renderer.render(View::NotFound(user_name.clone(), frame_name.clone())),
        };
    }

    queue
}

fn for_each_scale<C, G, I>(
    app_config: &AppConfig,
    single_scale_format: bool,
    consumer: &mut C,
    generator: G,
) where
    C: FnMut(I) -> (),
    G: Fn(String, f32) -> I,
{
    if single_scale_format {
        // For SVG images and SVG/XML icons always
        consumer(generator(String::new(), 1f32));
    } else {
        // For PNG, WEBP images only
        let scales: &HashMap<String, f32> = &app_config.android.images.scales;
        for (scale_name, scale_value) in scales {
            consumer(generator(scale_name.clone(), *scale_value));
        }
    }
}
