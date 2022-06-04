use crate::common::vdtool::error::VectorDrawableError;
use reqwest::StatusCode;
use thiserror::Error;

/// Common struct for all errors in this app.
#[derive(Debug, Error)]
pub enum AppError {
    ///
    /// Fetcher Errors
    ///

    #[error("Cannot read app config. Cause: {0}")]
    AppConfigOpen(String),

    #[error("Cannot parse app config. Cause: {0}")]
    AppConfigParse(#[from] serde_yaml::Error),

    #[error(
        "To export resources, you must specify `android.mainRes` or `android.images.mainRes` with `android.icons.mainRes` in {0}"
    )]
    AppConfigInvalidMainResCommon(String),

    #[error(
        "To export icons, you must specify `android.icons.mainRes` or `android.mainRes` in {0}"
    )]
    AppConfigInvalidMainResIcons(String),

    #[error(
        "To export images, you must specify `android.images.mainRes` or `android.mainRes` in {0}"
    )]
    AppConfigInvalidMainResImages(String),

    #[error("Cannot parse json response from Figma API ({0}).")]
    FetchDomResponseParsing(String),

    #[error("Cannot parse json response from Figma API ({0}).")]
    GetImageDownloadUrl(String),

    #[error("Can't get image byte stream. This error shouldn't have happened. Report it to the developer.")]
    GetImageByteStream,

    #[error("Can't save downloaded image into temp directory. Maybe something is wrong with the app permissions?")]
    GetImageTemporarySave,

    #[error(
        "Can't create temporary dirictory. Maybe something is wrong with the app permissions?"
    )]
    CreateTempDir,

    #[error("Can't access remote source {0}, {1}")]
    RequestHttpStatus(String, StatusCode),

    #[error("Invalid Figma personal access token: {0}")]
    RequestUnauthorized(StatusCode),

    #[error(
        "Can't access remote source: {0}. Check your internet connection, VPN settings and make sure the \
    address is reachable through your network."
    )]
    RequestMaybeVPN(String),

    #[error("Missing cache entry")]
    LoadFromCache,

    #[error("Can't save to cache")]
    SaveToCache,

    #[error("Can't find find frame with name `{0}`. Make sure such a frame exists.")]
    FindDesiredFrame(String),

    #[error("Desired frame `{0}` has no child frames. Add some pictures there :)")]
    DesiredFrameIsEmpty(String),

    ///
    /// WebP Converter
    ///

    #[error("Can't open temporary image file {0}")]
    SourceNotFound(String),

    #[error("Undelying reader error. Can't open temporary image file {0}")]
    UnderlyingReader(String),

    #[error("Can't decode temporary image file {0}")]
    CannotDecode(String),

    #[error("Can't encode temporary image file {0} because the image has unsupported color model (8-bit RGB and RGBA are supported now)")]
    CannotEncode(String),

    #[error("Can't write data to temporary webp image file. Cause: {0}")]
    WriteWebpTemporarySave(String),

    ///
    /// Export Common
    ///

    #[error("An image `{0}` is missing in frame `{1}`")]
    ImageMissingInFrame(String, String),

    #[error("Can't create res/drawable directory. Cause: {0}")]
    CannotCreateDrawableDir(String),

    #[error("Can't move image {0} from temporary directory to drawable directory. Cause: {1}")]
    CannotMoveToDrawableDir(String, String),

    #[error("Can't convert svg to android vector drawable xml. Cause: {0}")]
    CannotConvertToXml(#[from] VectorDrawableError),
}
