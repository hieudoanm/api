mod convert;
mod formats;
mod model;
mod openapi;

pub use convert::convert_collection;
pub use model::{
    ApiAuth, ApiBody, ApiCollection, ApiFolder, ApiParameter, ApiRequest, BodyKind, ClientFormat,
    ConversionFile, ConvertOptions, ParameterLocation,
};
pub use openapi::{convert_openapi, convert_openapi_all, parse_openapi};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiConvertError {
    #[error("failed to parse OpenAPI document: {0}")]
    Parse(String),
    #[error("invalid OpenAPI document: {0}")]
    Invalid(String),
    #[error("unsupported OpenAPI version: {0}")]
    UnsupportedVersion(String),
    #[error("conversion failed: {0}")]
    Conversion(String),
}

pub type Result<T> = std::result::Result<T, ApiConvertError>;
