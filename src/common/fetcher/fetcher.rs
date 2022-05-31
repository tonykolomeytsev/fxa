use std::collections::HashMap;

use crate::api::figma::{FigmaApi, FIGMA_FILES_ENDPOINT};
use crate::models::figma::Frame;
use crate::models::{config::AppConfig, figma::Document};

use crate::common::error::AppError;
use crate::common::fetcher::view::View;
use crate::common::renderer::Renderer;

pub struct FetcherEntry {
    pub app_config: AppConfig,
    pub document: Document,
    pub from_cache: bool,
    pub image_names_to_ids: HashMap<String, String>,
}

pub enum FetcherTarget {
    Images,
    Icons,
}

pub fn fetch(
    api: &FigmaApi,
    yaml_config_path: &String,
    fetcher_target: FetcherTarget,
    renderer: &Renderer,
) -> Result<FetcherEntry, AppError> {
    renderer.new_line();
    renderer.render(View::ReadingConfig {
        path: yaml_config_path.clone(),
    });
    let app_config = AppConfig::from_file(yaml_config_path)?;
    renderer.render(View::ReceivedConfig {
        path: yaml_config_path.clone(),
    });

    let document_url = format!("{}{}", FIGMA_FILES_ENDPOINT, &app_config.figma.file_id);
    renderer.render(View::FetchingDom {
        url: document_url.clone(),
    });
    let (document, from_cache) = fetch_dom(&api, &app_config)?;
    renderer.render(View::DomFetched {
        url: document_url.clone(),
        from_cache,
    });

    renderer.render(View::ProcessingDom);
    let names_to_ids = find_images_frame(&document, &app_config, fetcher_target)?;

    renderer.render(View::FoundImages {
        frame_name: app_config.common.images.figma_frame_name.clone(),
    });
    Ok(FetcherEntry {
        app_config,
        document,
        from_cache,
        image_names_to_ids: names_to_ids,
    })
}

fn fetch_dom(api: &FigmaApi, app_config: &AppConfig) -> Result<(Document, bool), AppError> {
    let file_id = &app_config.figma.file_id;
    api.get_document(&file_id)
}

fn find_images_frame(
    document: &Document,
    app_config: &AppConfig,
    fetcher_target: FetcherTarget,
) -> Result<HashMap<String, String>, AppError> {
    let desired_frame_name = match fetcher_target {
        FetcherTarget::Images => &app_config.common.images.figma_frame_name,
        FetcherTarget::Icons => &app_config.common.icons.figma_frame_name,
    };

    let frame = document
        .children
        .iter()
        .filter(|&canvas| {
            if let Some(desired_page_name) = &app_config.figma.page_name {
                desired_page_name == &canvas.name
            } else {
                true
            }
        })
        .flat_map(|canvas| &canvas.children)
        .find(|frame| &frame.name == desired_frame_name);

    if let Some(frame) = frame {
        let names_to_ids = collect_names_to_ids(&frame);
        if names_to_ids.is_empty() {
            Err(AppError::DesiredFrameIsEmpty(desired_frame_name.clone()))
        } else {
            Ok(names_to_ids)
        }
    } else {
        Err(AppError::FindDesiredFrame(desired_frame_name.clone()))
    }
}

fn collect_names_to_ids(frame: &Frame) -> HashMap<String, String> {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    if let Some(children) = &frame.children {
        for frame in children {
            hash_map.insert(frame.name.clone(), frame.id.clone());
        }
    }
    hash_map
}
