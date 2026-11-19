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
<title>{title} — Scalar</title>
</head>
<body>
<div id="app"></div>
<script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference@1.81.0/dist/browser/standalone.js"></script>
<script>
(function () {{
  var spec = {spec};
  Scalar.createApiReference("#app", {{ content: spec }});
}})();
</script>
</body>
</html>
"##
    );
    Ok(vec![ConversionFile {
        format: ClientFormat::Scalar,
        path: format!("{}.scalar.html", slugify(&name)),
        content,
    }])
}
