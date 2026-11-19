use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientFormat {
    Postman,
    Bruno,
    Insomnia,
    Hoppscotch,
    RestClient,
    ThunderClient,
    Yaak,
    Hurl,
    Kreya,
    Curl,
}

impl ClientFormat {
    pub const ALL: [Self; 10] = [
        Self::Postman,
        Self::Bruno,
        Self::Insomnia,
        Self::Hoppscotch,
        Self::RestClient,
        Self::ThunderClient,
        Self::Yaak,
        Self::Hurl,
        Self::Kreya,
        Self::Curl,
    ];

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "postman" | "postman-collection" => Some(Self::Postman),
            "bruno" | "bru" => Some(Self::Bruno),
            "insomnia" => Some(Self::Insomnia),
            "hoppscotch" => Some(Self::Hoppscotch),
            "rest-client" | "restclient" | "http" => Some(Self::RestClient),
            "thunder-client" | "thunderclient" | "thunder" | "tc" => Some(Self::ThunderClient),
            "yaak" => Some(Self::Yaak),
            "hurl" => Some(Self::Hurl),
            "kreya" => Some(Self::Kreya),
            "curl" | "curl.sh" | "sh" | "shell" => Some(Self::Curl),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Postman => "postman",
            Self::Bruno => "bruno",
            Self::Insomnia => "insomnia",
            Self::Hoppscotch => "hoppscotch",
            Self::RestClient => "rest-client",
            Self::ThunderClient => "thunder-client",
            Self::Yaak => "yaak",
            Self::Hurl => "hurl",
            Self::Kreya => "kreya",
            Self::Curl => "curl",
        }
    }

    pub fn file_name(self, collection_name: &str) -> String {
        match self {
            Self::Postman => format!("{collection_name}.postman_collection.json"),
            Self::Insomnia => format!("{collection_name}.insomnia.json"),
            Self::Hoppscotch => format!("{collection_name}.hoppscotch.json"),
            Self::RestClient => format!("{collection_name}.http"),
            Self::ThunderClient => format!("thunder-collection_{collection_name}.json"),
            Self::Curl => format!("{collection_name}.sh"),
            Self::Bruno | Self::Yaak | Self::Hurl | Self::Kreya => "collection".to_string(),
        }
    }
}

impl fmt::Display for ClientFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConvertOptions {
    pub collection_name: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConversionFile {
    pub format: ClientFormat,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ApiCollection {
    pub name: String,
    pub description: String,
    pub version: String,
    pub servers: Vec<String>,
    pub folders: Vec<ApiFolder>,
}

#[derive(Debug, Clone)]
pub struct ApiFolder {
    pub name: String,
    pub requests: Vec<ApiRequest>,
}

#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub id: String,
    pub name: String,
    pub method: String,
    pub path: String,
    pub server: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub path_params: Vec<ApiParameter>,
    pub query_params: Vec<ApiParameter>,
    pub headers: Vec<ApiParameter>,
    pub body: Option<ApiBody>,
    pub auth: Vec<ApiAuth>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterLocation {
    Query,
    Path,
    Header,
    Cookie,
}

impl ParameterLocation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Path => "path",
            Self::Header => "header",
            Self::Cookie => "cookie",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiParameter {
    pub name: String,
    pub location: ParameterLocation,
    pub required: bool,
    pub description: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyKind {
    Json,
    UrlEncoded,
    FormData,
    Text,
    Other,
}

#[derive(Debug, Clone)]
pub struct ApiBody {
    pub content_type: String,
    pub kind: BodyKind,
    pub raw: Option<String>,
    pub fields: Vec<ApiParameter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiAuth {
    Bearer {
        token: String,
    },
    Basic {
        username: String,
        password: String,
    },
    ApiKey {
        name: String,
        location: ParameterLocation,
        value: String,
    },
    OAuth2 {
        token: String,
    },
    Other {
        kind: String,
        name: String,
    },
}

impl ApiAuth {
    pub fn is_placeholder(&self) -> bool {
        match self {
            Self::Bearer { token } | Self::OAuth2 { token } => token.contains("{{"),
            Self::Basic { username, password } => {
                username.contains("{{") || password.contains("{{")
            }
            Self::ApiKey { value, .. } => value.contains("{{"),
            Self::Other { .. } => false,
        }
    }
}
