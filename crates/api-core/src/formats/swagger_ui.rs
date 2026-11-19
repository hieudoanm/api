use crate::formats::util::{collection_name, escape_html, slugify, spec_literal};
use crate::{ApiCollection, ClientFormat, ConversionFile, ConvertOptions, Result};

pub fn convert(
    collection: &ApiCollection,
    options: &ConvertOptions,
) -> Result<Vec<ConversionFile>> {
    let name = collection_name(collection, options);
    let title = escape_html(&name);
    let spec = spec_literal(&collection.spec);
    let content = format!(
        r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title} — Swagger UI</title>
<link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.35.1/swagger-ui.css">
</head>
<body>
<div id="ui"></div>
<script src="https://unpkg.com/swagger-ui-dist@5.35.1/swagger-ui-bundle.js"></script>
<script src="https://unpkg.com/swagger-ui-dist@5.35.1/swagger-ui-standalone-preset.js"></script>
<script>
window.onload = function () {{
  window.ui = SwaggerUIBundle({{
    dom_id: "#ui",
    spec: {spec},
    presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
    layout: "BaseLayout"
  }});
}};
</script>
</body>
</html>
"##
    );
    Ok(vec![ConversionFile {
        format: ClientFormat::SwaggerUi,
        path: format!("{}.swagger-ui.html", slugify(&name)),
        content,
    }])
}
