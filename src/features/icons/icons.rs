use crate::api::figma::FigmaApi;
use crate::common::error::AppError;
use crate::common::fetching::{fetch, FetcherTarget};
use crate::common::fileutils::{create_dir, move_file};
use crate::common::gathering::gathering::gather_names;
use crate::common::http_client::create_http_client;
use crate::common::renderer::Renderer;
use crate::common::res_name::to_res_name;
use crate::common::vdtool::vdtool::convert_svg_to_xml;
use crate::feature_icons::view::View;
use crate::models::config::{AppConfig, IconFormat, ImageFormat};

#[derive(Debug, Clone)]
struct IconInfo {
    id: String,
    user_name: String,
    format: IconFormat,
    res: ResourceInfo,
}

#[derive(Debug, Clone)]
struct ResourceInfo {
    name: String,
    night: bool,
}

impl IconInfo {
    fn drawable_dir_name(&self) -> String {
        if self.res.night {
            "drawable-night".to_string()
        } else {
            "drawable".to_string()
        }
    }
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

    let frame_name = &app_config.common.icons.figma_frame_name;
    let format = &app_config.android.icons.format;
    let icons_for_export: Vec<IconInfo> = gather_names(
        &app_config,
        &frame_name,
        &image_names,
        &names_to_ids,
        true,
        |e| IconInfo {
            id: e.figma_id,
            user_name: e.user_name.clone(),
            format: format.clone(),
            res: ResourceInfo {
                name: to_res_name(&e.user_name),
                night: e.night,
            },
        },
    );

    for icon in icons_for_export {
        let export_result = export_icon(&api, &app_config, &icon, &renderer);

        match export_result {
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
    icon: &IconInfo,
    renderer: &Renderer,
) -> Result<(), AppError> {
    let file_id = &app_config.figma.file_id;
    let node_id = &icon.id;

    // Get download url for exported icon
    renderer.render(View::FetchingIcon(
        icon.user_name.clone(),
        icon.drawable_dir_name(),
    ));
    let icon_download_url =
        api.get_image_download_url(file_id, node_id, 1.0f32, &ImageFormat::Svg)?;

    // Download icon from gotten url to app's TEMPORARY dir
    renderer.render(View::DownloadingIcon(
        icon.user_name.clone(),
        icon.drawable_dir_name(),
    ));
    let icon_temporary_file_name = api.get_image(
        &icon_download_url,
        &icon.res.name,
        &icon.drawable_dir_name(),
        &ImageFormat::Svg,
    )?;

    // Convert to VectorDrawable XML
    let icon_temporary_file_name =
        convert_to_vector_drawable(&icon, &icon_temporary_file_name, &renderer)?;

    // Create drawable dir in res dir of android project
    renderer.render(View::IconDownloaded(
        icon.user_name.clone(),
        icon.drawable_dir_name(),
    ));
    let res_path = &app_config
        .main_res_icons()
        .expect("Validation is done in fetcher");
    let full_final_icon_dir = format!("{}/drawable", &res_path);
    create_dir(&full_final_icon_dir)
        .map_err(|e| AppError::CannotCreateDrawableDir(format!("{}", e)))?;

    // Move icon from temporary dir to drawable dir of android project
    let extension = icon.format.extension();
    let full_final_icon_path =
        format!("{}/{}.{}", full_final_icon_dir, &icon.res.name, &extension,);
    move_file(&icon_temporary_file_name, &full_final_icon_path)
        .map_err(|e| AppError::CannotMoveToDrawableDir(icon.user_name.clone(), format!("{}", e)))?;

    // Tell the user that we are done
    renderer.render(View::IconExported(
        icon.user_name.clone(),
        icon.drawable_dir_name(),
    ));
    Ok(())
}

fn convert_to_vector_drawable(
    icon: &IconInfo,
    icon_file_name: &String,
    renderer: &Renderer,
) -> Result<String, AppError> {
    match icon.format {
        IconFormat::Xml => {
            renderer.render(View::ConvertingToXml(
                icon.user_name.clone(),
                icon.drawable_dir_name(),
            ));
            let new_icon_path =
                convert_svg_to_xml(icon_file_name).map_err(AppError::CannotConvertToXml)?;
            renderer.render(View::ConvertedToXml(
                icon.user_name.clone(),
                icon.drawable_dir_name(),
            ));
            Ok(new_icon_path)
        }
        IconFormat::Svg => Ok(icon_file_name.clone()),
    }
}
