use std::collections::HashMap;

use crate::api::figma::FigmaApi;
use crate::common::error::AppError;
use crate::common::fetching::{fetch, FetcherTarget};
use crate::common::fileutils::{create_dir, move_file};
use crate::common::http_client::create_http_client;
use crate::common::renderer::Renderer;
use crate::common::res_name::to_res_name;
use crate::common::suggestions::generate_name_suggections;
use crate::common::vdtool::vdtool::convert_svg_to_xml;
use crate::feature_icons::view::View;
use crate::models::config::{AppConfig, IconFormat, ImageFormat};

#[derive(Debug, Clone)]
struct IconInfo {
    name: String,
    res_name: String,
    format: IconFormat,
}

pub fn export_icons(token: &String, image_names: &Vec<String>, yaml_config_path: &String) {
    let renderer = Renderer();
    let api = FigmaApi::new(create_http_client(&token));

    let fetcher_entry = match fetch(&api, &yaml_config_path, FetcherTarget::Icons, &renderer) {
        Ok(fetcher_entry) => fetcher_entry,
        Err(e) => {
            renderer.render(View::Error(format!("{}", e)));
            return;
        }
    };
    let (app_config, names_to_ids) = (fetcher_entry.app_config, fetcher_entry.image_names_to_ids);

    for icon_name in image_names {
        // Just to not to pass long parameter list to export_icon function
        let icon_info = IconInfo {
            name: icon_name.clone(),
            res_name: to_res_name(&icon_name),
            format: app_config.android.icons.format.clone(),
        };
        match export_icon(&api, &app_config, &icon_info, &names_to_ids, &renderer) {
            Err(AppError::ImageMissingInFrame(name, frame, Some(suggestions))) => renderer
                .render(View::ErrorWithSuggestions(
                    format!("An icon `{}` is missing in frame `{}`, but there are icons with similar names:", name, frame),
                    suggestions,
                )),
            Err(e) => renderer.render(View::Error(e.to_string())),
            Ok(()) => (),
        }
        renderer.new_line();
    }

    renderer.render(View::Done { message: None });
}

fn export_icon(
    api: &FigmaApi,
    app_config: &AppConfig,
    icon_info: &IconInfo,
    names_to_ids: &HashMap<String, String>,
    renderer: &Renderer,
) -> Result<(), AppError> {
    let file_id = &app_config.figma.file_id;

    // Find icon frame id by its name
    let node_id = names_to_ids.get(&icon_info.name).ok_or_else(|| {
        // If we can't find desired icon by name, offer a suggestions
        let frame_name = &app_config.common.icons.figma_frame_name;
        let available_names = names_to_ids
            .iter()
            .map(|(k, _)| k.clone())
            .collect::<Vec<String>>();
        let suggestions = generate_name_suggections(&icon_info.name, &available_names);
        AppError::ImageMissingInFrame(icon_info.name.clone(), frame_name.clone(), suggestions)
    })?;

    // Get download url for exported icon
    renderer.render(View::FetchingIcon(icon_info.name.clone()));
    let icon_download_url =
        api.get_image_download_url(file_id, node_id, 1.0f32, &ImageFormat::Svg)?;

    // Download icon from gotten url to app's TEMPORARY dir
    renderer.render(View::DownloadingIcon(icon_info.name.clone()));
    let icon_temporary_file_name = api.get_image(
        &icon_download_url,
        &icon_info.res_name,
        &String::new(),
        &ImageFormat::Svg,
    )?;

    // Convert to VectorDrawable XML
    let icon_temporary_file_name =
        convert_to_vector_drawable(&icon_info, &icon_temporary_file_name, &renderer)?;

    // Create drawable dir in res dir of android project
    renderer.render(View::IconDownloaded(icon_info.name.clone()));
    let res_path = &app_config
        .main_res_icons()
        .expect("Validation is done in fetcher");
    let full_final_icon_dir = format!("{}/drawable", &res_path);
    create_dir(&full_final_icon_dir)
        .map_err(|e| AppError::CannotCreateDrawableDir(format!("{}", e)))?;

    // Move icon from temporary dir to drawable dir of android project
    let extension = icon_info.format.extension();
    let full_final_icon_path = format!(
        "{}/{}.{}",
        full_final_icon_dir, &icon_info.res_name, &extension,
    );
    move_file(&icon_temporary_file_name, &full_final_icon_path)
        .map_err(|e| AppError::CannotMoveToDrawableDir(icon_info.name.clone(), format!("{}", e)))?;

    // Tell the user that we are done
    renderer.render(View::IconExported(icon_info.name.clone()));
    Ok(())
}

fn convert_to_vector_drawable(
    icon_info: &IconInfo,
    icon_file_name: &String,
    renderer: &Renderer,
) -> Result<String, AppError> {
    match icon_info.format {
        IconFormat::Xml => {
            renderer.render(View::ConvertingToXml(icon_info.name.clone()));
            let new_icon_path =
                convert_svg_to_xml(icon_file_name).map_err(AppError::CannotConvertToXml)?;
            renderer.render(View::ConvertedToXml(icon_info.name.clone()));
            Ok(new_icon_path)
        }
        IconFormat::Svg => Ok(icon_file_name.clone()),
    }
}
