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
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title} — Stoplight Elements</title>
<link rel="stylesheet" href="https://unpkg.com/@stoplight/elements@9.0.29/styles.min.css">
</head>
<body>
<elements-api id="api" layout="sidebar" router="hash"></elements-api>
<script src="https://unpkg.com/@stoplight/elements@9.0.29/web-components.min.js"></script>
<script>
(function () {{
  var spec = {spec};
  customElements.whenDefined("elements-api").then(function () {{
    document.getElementById("api").apiDescriptionDocument = spec;
  }});
}})();
</script>
</body>
</html>
"#
    );
    Ok(vec![ConversionFile {
        format: ClientFormat::Stoplight,
        path: format!("{}.stoplight.html", slugify(&name)),
        content,
    }])
}
