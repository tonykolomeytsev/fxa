use std::collections::HashMap;

use crate::api::figma::FigmaApi;
use crate::common::error::AppError;
use crate::common::fetcher::{fetch, FetcherTarget};
use crate::common::fileutils::{create_dir, move_file};
use crate::common::http_client::create_http_client;
use crate::common::renderer::Renderer;
use crate::common::res_name::to_res_name;
use crate::common::vdtool::svg2vector::convert_svg_to_xml;
use crate::feature_icons::view::View;
use crate::models::config::{AppConfig, ImageFormat};

#[derive(Debug, Clone)]
struct IconInfo {
    name: String,
    res_name: String,
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

    for image_name in image_names {
        // Just to not to pass long parameter list to export_icon function
        let icon_info = IconInfo {
            name: image_name.clone(),
            res_name: to_res_name(&image_name),
        };
        if let Err(e) = export_icon(&api, &app_config, &icon_info, &names_to_ids, &renderer) {
            renderer.render(View::Error(format!("{}", e)))
        };
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
    let frame_name = &app_config.common.images.figma_frame_name;

    // Find image frame id by its name
    let node_id = names_to_ids
        .get(&icon_info.name)
        .ok_or_else(|| AppError::ImageMissingInFrame(icon_info.name.clone(), frame_name.clone()))?;

    // Get download url for exported image
    renderer.render(View::FetchingIcon(icon_info.name.clone()));
    let image_download_url =
        api.get_image_download_url(file_id, node_id, 1.0f32, &ImageFormat::Svg)?;

    // Download image from gotten url to app's TEMPORARY dir
    renderer.render(View::DownloadingIcon(icon_info.name.clone()));
    let image_temporary_file_name = api.get_image(
        &image_download_url,
        &icon_info.res_name,
        &String::new(),
        &ImageFormat::Svg,
    )?;

    // Convert to VectorDrawable XML
    let image_temporary_file_name =
        convert_to_vector_drawable(&icon_info, &image_temporary_file_name)?;

    // Create drawable dir in res dir of android project
    renderer.render(View::IconDownloaded(icon_info.name.clone()));
    let res_path = &app_config.android.main_res;
    let full_final_image_dir = format!("{}/drawable", &res_path);
    create_dir(&full_final_image_dir)
        .map_err(|e| AppError::CannotCreateDrawableDir(format!("{}", e)))?;

    // Move image from temporary dir to drawable dir of android project
    let full_final_image_path = format!("{}/{}.svg", full_final_image_dir, &icon_info.res_name);
    move_file(&image_temporary_file_name, &full_final_image_path)
        .map_err(|e| AppError::CannotMoveToDrawableDir(icon_info.name.clone(), format!("{}", e)))?;

    // Tell the user that we are done
    renderer.render(View::IconExported(icon_info.name.clone()));
    Ok(())
}

fn convert_to_vector_drawable(
    icon_info: &IconInfo,
    image_file_name: &String,
) -> Result<String, AppError> {
    convert_svg_to_xml(image_file_name).unwrap();
    Ok(image_file_name.clone())
}
