use crate::api::figma::FigmaApi;
use crate::common::error::AppError;
use crate::common::fetching::{fetch, FetcherTarget};
use crate::common::fileutils::{create_dir, move_file};
use crate::common::gathering::gathering::gather_names;
use crate::common::http_client::create_http_client;
use crate::common::renderer::Renderer;
use crate::common::res_name::to_res_name;
use crate::common::webp;
use crate::feature_images::view::View;
use crate::models::config::{AppConfig, ImageFormat};

#[derive(Debug, Clone)]
struct ImageInfo {
    id: String,
    user_name: String,
    format: ImageFormat,
    res: ResourceInfo,
}

#[derive(Debug, Clone)]
struct ResourceInfo {
    name: String,
    scale: ScaleInfo,
    night: bool,
}

#[derive(Debug, Clone)]
struct ScaleInfo {
    name: String,
    value: f32,
}

impl ImageInfo {
    fn drawable_dir_name(&self) -> String {
        match (self.format.is_svg(), self.res.night) {
            (true, true) => "drawable-night".to_string(),
            (true, false) => "drawable".to_string(),
            (false, true) => format!("drawable-night-{}", self.res.scale.name),
            (false, false) => format!("drawable-{}", self.res.scale.name),
        }
    }
}

pub fn export_images(token: &String, image_names: &[String], yaml_config_path: &String) {
    let renderer = Renderer();
    let api = FigmaApi::new(create_http_client(&token));

    let fetcher_entry = match fetch(&api, &yaml_config_path, FetcherTarget::Images, &renderer) {
        Ok(fetcher_entry) => fetcher_entry,
        Err(e) => {
            renderer.render(View::Error(format!("{}", e)));
            return;
        }
    };
    let (app_config, names_to_ids) = (fetcher_entry.app_config, fetcher_entry.image_names_to_ids);

    let frame_name = &app_config.common.images.figma_frame_name;
    let format = &app_config.android.images.format;
    let single_scale_format = format.is_svg();
    let images_for_export: Vec<ImageInfo> = gather_names(
        &app_config,
        &frame_name,
        &image_names,
        &names_to_ids,
        single_scale_format,
        |e| ImageInfo {
            id: e.figma_id,
            user_name: e.user_name.clone(),
            format: format.clone(),
            res: ResourceInfo {
                name: to_res_name(&e.user_name),
                scale: ScaleInfo {
                    name: e.scale_name.clone(),
                    value: e.scale_value,
                },
                night: e.night,
            },
        },
    );

    for image in images_for_export {
        let export_result = export_image(&api, &app_config, &image, &renderer);

        match &export_result {
            Err(e) => renderer.render(View::Error(e.to_string())),
            Ok(()) => (),
        }

        renderer.new_line();
    }

    renderer.render(View::Done { message: None });
}

fn export_image(
    api: &FigmaApi,
    app_config: &AppConfig,
    image: &ImageInfo,
    renderer: &Renderer,
) -> Result<(), AppError> {
    let file_id = &app_config.figma.file_id;
    let quality = app_config.android.images.webp_options.quality;
    let node_id = &image.id;

    // Get download url for exported image
    renderer.render(View::FetchingImage(
        image.user_name.clone(),
        image.drawable_dir_name(),
    ));
    let image_download_url =
        api.get_image_download_url(file_id, node_id, image.res.scale.value, &image.format)?;

    // Download image from gotten url to app's TEMPORARY dir
    renderer.render(View::DownloadingImage(
        image.user_name.clone(),
        image.drawable_dir_name(),
    ));
    let image_format = &app_config.android.images.format;
    let image_temporary_file_name = api.get_image(
        &image_download_url,
        &image.res.name,
        &image.res.scale.name,
        &image_format,
    )?;

    // So... Convert if necessary :)
    let image_temporary_file_name =
        convert_to_webp_if_necessary(&image, image_temporary_file_name, quality, &renderer)?;

    // Create drawable-XXXX dir in res dir of android project
    let res_dir = &app_config
        .main_res_images()
        .expect("Validation is done in fetcher");
    let full_final_image_dir = format!("{}/{}", &res_dir, image.drawable_dir_name());
    create_dir(&full_final_image_dir)
        .map_err(|e| AppError::CannotCreateDrawableDir(e.to_string()))?;

    // Move image from temporary dir to drawable dir of android project
    let extension = image.format.extension();
    let full_final_image_path = format!(
        "{}/{}.{}",
        full_final_image_dir, &image.res.name, &extension
    );
    move_file(&image_temporary_file_name, &full_final_image_path)
        .map_err(|e| AppError::CannotMoveToDrawableDir(image.user_name.clone(), e.to_string()))?;

    // Tell the user that we are done exporting image for this scale
    renderer.render(View::ImageExported(
        image.user_name.clone(),
        image.drawable_dir_name(),
    ));
    Ok(())
}

fn convert_to_webp_if_necessary(
    image: &ImageInfo,
    image_file_name: String,
    quality: f32,
    renderer: &Renderer,
) -> Result<String, AppError> {
    match image.format {
        ImageFormat::Webp => {
            renderer.render(View::ConvertingToWebp(
                image.user_name.clone(),
                image.drawable_dir_name(),
            ));
            let new_image_path = webp::image_to_webp(&image_file_name, quality)?;
            renderer.render(View::ConvertedToWebp(
                image.user_name.clone(),
                image.drawable_dir_name(),
            ));
            Ok(new_image_path)
        }
        _ => Ok(image_file_name),
    }
}
