use api_core::{ApiAuth, BodyKind, ClientFormat, ParameterLocation};

#[test]
fn client_format_parse_aliases() {
    let cases = [
        ("postman", ClientFormat::Postman),
        ("postman-collection", ClientFormat::Postman),
        ("bruno", ClientFormat::Bruno),
        ("bru", ClientFormat::Bruno),
        ("insomnia", ClientFormat::Insomnia),
        ("hoppscotch", ClientFormat::Hoppscotch),
        ("rest-client", ClientFormat::RestClient),
        ("restclient", ClientFormat::RestClient),
        ("http", ClientFormat::RestClient),
        ("thunder-client", ClientFormat::ThunderClient),
        ("thunderclient", ClientFormat::ThunderClient),
        ("thunder", ClientFormat::ThunderClient),
        ("tc", ClientFormat::ThunderClient),
        ("yaak", ClientFormat::Yaak),
        ("hurl", ClientFormat::Hurl),
        ("kreya", ClientFormat::Kreya),
        ("curl", ClientFormat::Curl),
        ("curl.sh", ClientFormat::Curl),
        ("sh", ClientFormat::Curl),
        ("shell", ClientFormat::Curl),
    ];
    for (input, expected) in cases {
        assert_eq!(ClientFormat::parse(input), Some(expected), "{input}");
    }
}

#[test]
fn client_format_parse_is_case_insensitive_and_trims() {
    assert_eq!(
        ClientFormat::parse("  PostMan "),
        Some(ClientFormat::Postman)
    );
    assert_eq!(ClientFormat::parse("CURL"), Some(ClientFormat::Curl));
}

#[test]
fn client_format_parse_rejects_unknown() {
    assert_eq!(ClientFormat::parse(""), None);
    assert_eq!(ClientFormat::parse("postman2"), None);
    assert_eq!(ClientFormat::parse("yaml"), None);
}

#[test]
fn client_format_as_str_roundtrips() {
    for format in ClientFormat::ALL {
        assert_eq!(ClientFormat::parse(format.as_str()), Some(format));
    }
    assert_eq!(ClientFormat::Postman.as_str(), "postman");
    assert_eq!(ClientFormat::RestClient.as_str(), "rest-client");
    assert_eq!(ClientFormat::Curl.as_str(), "curl");
}

#[test]
fn client_format_display_matches_as_str() {
    for format in ClientFormat::ALL {
        assert_eq!(format.to_string(), format.as_str());
    }
}

#[test]
fn client_format_file_names() {
    assert_eq!(
        ClientFormat::Postman.file_name("Acme API"),
        "Acme API.postman_collection.json"
    );
    assert_eq!(
        ClientFormat::Insomnia.file_name("Acme"),
        "Acme.insomnia.json"
    );
    assert_eq!(
        ClientFormat::Hoppscotch.file_name("Acme"),
        "Acme.hoppscotch.json"
    );
    assert_eq!(ClientFormat::RestClient.file_name("Acme"), "Acme.http");
    assert_eq!(
        ClientFormat::ThunderClient.file_name("Acme"),
        "thunder-collection_Acme.json"
    );
    assert_eq!(ClientFormat::Curl.file_name("Acme"), "Acme.sh");
    for format in [
        ClientFormat::Bruno,
        ClientFormat::Yaak,
        ClientFormat::Hurl,
        ClientFormat::Kreya,
    ] {
        assert_eq!(format.file_name("Acme"), "collection");
    }
}

#[test]
fn parameter_location_as_str() {
    assert_eq!(ParameterLocation::Query.as_str(), "query");
    assert_eq!(ParameterLocation::Path.as_str(), "path");
    assert_eq!(ParameterLocation::Header.as_str(), "header");
    assert_eq!(ParameterLocation::Cookie.as_str(), "cookie");
}

#[test]
fn auth_is_placeholder_detection() {
    assert!(ApiAuth::Bearer {
        token: "{{accessToken}}".to_string()
    }
    .is_placeholder());
    assert!(!ApiAuth::Bearer {
        token: "abc".to_string()
    }
    .is_placeholder());
    assert!(ApiAuth::OAuth2 {
        token: "{{accessToken}}".to_string()
    }
    .is_placeholder());
    assert!(ApiAuth::Basic {
        username: "{{username}}".to_string(),
        password: "x".to_string()
    }
    .is_placeholder());
    assert!(!ApiAuth::Basic {
        username: "u".to_string(),
        password: "p".to_string()
    }
    .is_placeholder());
    assert!(ApiAuth::ApiKey {
        name: "k".to_string(),
        location: ParameterLocation::Header,
        value: "{{apiKey}}".to_string()
    }
    .is_placeholder());
    assert!(!ApiAuth::ApiKey {
        name: "k".to_string(),
        location: ParameterLocation::Query,
        value: "literal".to_string()
    }
    .is_placeholder());
    assert!(!ApiAuth::Other {
        kind: "mutualTLS".to_string(),
        name: "mtls".to_string()
    }
    .is_placeholder());
}

#[test]
fn body_kind_variants_exist() {
    assert_ne!(BodyKind::Json, BodyKind::UrlEncoded);
    assert_ne!(BodyKind::FormData, BodyKind::Text);
    assert_ne!(BodyKind::Other, BodyKind::Json);
}
