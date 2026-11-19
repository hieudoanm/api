use crate::formats;
use crate::{ApiCollection, ClientFormat, ConversionFile, ConvertOptions, Result};

pub fn convert_collection(
    collection: &ApiCollection,
    format: ClientFormat,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    match format {
        ClientFormat::Postman => formats::postman::convert(collection, options),
        ClientFormat::Bruno => formats::bruno::convert(collection, options),
        ClientFormat::Insomnia => formats::insomnia::convert(collection, options),
        ClientFormat::Hoppscotch => formats::hoppscotch::convert(collection, options),
        ClientFormat::RestClient => formats::rest_client::convert(collection, options),
        ClientFormat::ThunderClient => formats::thunder_client::convert(collection, options),
        ClientFormat::Yaak => formats::yaak::convert(collection, options),
        ClientFormat::Hurl => formats::hurl::convert(collection, options),
        ClientFormat::Kreya => formats::kreya::convert(collection, options),
        ClientFormat::Curl => formats::curl::convert(collection, options),
        ClientFormat::SwaggerUi => formats::swagger_ui::convert(collection, options),
        ClientFormat::Redoc => formats::redoc::convert(collection, options),
        ClientFormat::Scalar => formats::scalar::convert(collection, options),
        ClientFormat::Stoplight => formats::stoplight::convert(collection, options),
        ClientFormat::RapiDoc => formats::rapidoc::convert(collection, options),
    }
}
