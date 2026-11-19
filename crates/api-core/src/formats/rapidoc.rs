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
<title>{title} — RapiDoc</title>
</head>
<body>
<rapi-doc id="doc" theme="light"></rapi-doc>
<script type="module">
import "https://unpkg.com/rapidoc@14.1.0";
(function () {{
  var spec = {spec};
  var url = URL.createObjectURL(
    new Blob([JSON.stringify(spec)], {{ type: "application/json" }})
  );
  customElements.whenDefined("rapi-doc").then(function () {{
    document.getElementById("doc").setAttribute("spec-url", url);
  }});
}})();
</script>
</body>
</html>
"#
    );
    Ok(vec![ConversionFile {
        format: ClientFormat::RapiDoc,
        path: format!("{}.rapidoc.html", slugify(&name)),
        content,
    }])
}
