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
<title>{title} — ReDoc</title>
</head>
<body>
<div id="redoc"></div>
<script src="https://cdn.jsdelivr.net/npm/redoc@2.5.4/bundles/redoc.standalone.js"></script>
<script>
(function () {{
  var spec = {spec};
  var url = URL.createObjectURL(
    new Blob([JSON.stringify(spec)], {{ type: "application/json" }})
  );
  Redoc.init(url, document.getElementById("redoc"), {{}});
}})();
</script>
</body>
</html>
"#
    );
    Ok(vec![ConversionFile {
        format: ClientFormat::Redoc,
        path: format!("{}.redoc.html", slugify(&name)),
        content,
    }])
}
