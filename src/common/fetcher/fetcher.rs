use std::collections::HashMap;

use crate::api::figma::{FigmaApi, FIGMA_FILES_ENDPOINT};
use crate::models::figma::Frame;
use crate::models::{config::AppConfig, figma::Document};

use crate::common::error::CommonError;
use crate::common::fetcher::view::View;
use crate::common::renderer::Renderer;

pub struct FetcherEntry {
    pub app_config: AppConfig,
    pub document: Document,
    pub from_cache: bool,
    pub images_name_to_id: HashMap<String, String>,
}

pub fn fetch(
    api: &FigmaApi,
    yaml_config_path: &String,
    renderer: &Renderer,
) -> Result<FetcherEntry, CommonError> {
    let mut document_url = String::new();

    renderer.new_line();
    renderer.render(View::ReadingConfig {
        path: yaml_config_path.clone(),
    });
    AppConfig::from_file(yaml_config_path)
        .map(|app_config| {
            renderer.render(View::ReceivedConfig {
                path: yaml_config_path.clone(),
            });
            document_url = format!("{}{}", FIGMA_FILES_ENDPOINT, &app_config.figma.file_id);
            app_config
        })
        .and_then(|app_config| {
            renderer.render(View::FetchingDom {
                url: document_url.clone(),
            });
            fetch_dom(&api, &app_config).map(|(doc, from_cache)| (app_config, doc, from_cache))
        })
        .and_then(|(app_config, doc, from_cache)| {
            renderer.render(View::DomFetched {
                url: document_url.clone(),
                from_cache,
            });
            renderer.render(View::ProcessingDom);
            find_images_frame(&doc, app_config).map(|(app_config, images_name_to_id)| {
                renderer.render(View::FoundImages {
                    frame_name: app_config.common.images.figma_frame_name.clone(),
                });
                FetcherEntry {
                    app_config,
                    document: doc,
                    from_cache,
                    images_name_to_id,
                }
            })
        })
}

fn fetch_dom(api: &FigmaApi, app_config: &AppConfig) -> Result<(Document, bool), CommonError> {
    let file_id = &app_config.figma.file_id;
    api.get_document(&file_id)
}

fn find_images_frame(
    document: &Document,
    app_config: AppConfig,
) -> Result<(AppConfig, HashMap<String, String>), CommonError> {
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
        .find(|frame| &frame.name == &app_config.common.images.figma_frame_name);
    if let Some(frame) = frame {
        Ok((app_config, map_images_name_to_id(&frame)))
    } else {
        let message = format!(
            "during search frame with name `{}`",
            &app_config.common.images.figma_frame_name,
        );
        let cause = Some("Make sure such a frame exists".to_string());
        Err(CommonError { message, cause })
    }
}

fn map_images_name_to_id(frame: &Frame) -> HashMap<String, String> {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    match &frame.children {
        Some(children) => {
            children.iter().for_each(|frame| {
                hash_map.insert(frame.name.clone(), frame.id.clone());
            });
        }
        None => (),
    };
    hash_map
}
