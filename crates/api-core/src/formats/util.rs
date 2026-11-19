use crate::{ApiCollection, ApiRequest, ConvertOptions};

pub fn collection_name(collection: &ApiCollection, options: &ConvertOptions) -> String {
    options
        .collection_name
        .clone()
        .unwrap_or_else(|| collection.name.clone())
        .trim()
        .to_string()
}

pub fn base_url(collection: &ApiCollection, options: &ConvertOptions) -> String {
    if let Some(url) = options
        .base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return url.trim_end_matches('/').to_string();
    }
    collection
        .servers
        .first()
        .cloned()
        .unwrap_or_else(|| "https://api.example.com".to_string())
}

pub fn request_server<'a>(
    request: &'a ApiRequest,
    collection: &'a ApiCollection,
    options: &'a ConvertOptions,
) -> String {
    if let Some(url) = options
        .base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return url.trim_end_matches('/').to_string();
    }
    request
        .server
        .clone()
        .or_else(|| collection.servers.first().cloned())
        .unwrap_or_else(|| "https://api.example.com".to_string())
}

pub fn client_path(path: &str) -> String {
    let mut result = path.to_string();
    for (name, _) in path_param_names(path) {
        result = result.replace(&format!("{{{name}}}"), &format!(":{name}"));
    }
    result
}

pub fn path_param_names(path: &str) -> Vec<(String, String)> {
    let mut names = Vec::new();
    let mut rest = path;
    while let Some(start) = rest.find('{') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('}') else {
            break;
        };
        let name = after[..end].trim().to_string();
        if !name.is_empty() && !names.iter().any(|(n, _)| n == &name) {
            names.push((name.clone(), name));
        }
        rest = &after[end + 1..];
    }
    names
}

pub fn is_json_content_type(content_type: &str) -> bool {
    let lower = content_type.to_ascii_lowercase();
    lower.contains("json") || lower.ends_with("+json")
}

pub fn auth_for_request(request: &ApiRequest) -> Option<&crate::ApiAuth> {
    request.auth.first()
}

pub fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = true;
    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }
    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "request".to_string()
    } else {
        slug
    }
}

pub fn unique_slug(base: &str, used: &mut Vec<String>) -> String {
    let mut candidate = slugify(base);
    let mut index = 2;
    while used.iter().any(|item| item == &candidate) {
        candidate = format!("{}-{index}", slugify(base));
        index += 1;
    }
    used.push(candidate.clone());
    candidate
}

pub fn stable_id(prefix: &str, seed: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in seed.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut out = String::with_capacity(prefix.len() + 1 + 10);
    out.push_str(prefix);
    out.push('_');
    for shift in (0..10).rev() {
        let index = ((hash >> (shift * 6)) & 0x3f) as usize % ALPHABET.len();
        out.push(ALPHABET[index] as char);
    }
    out
}

pub fn pseudo_uuid(seed: &str) -> String {
    let mut digits = String::new();
    let mut counter = 0usize;
    while digits.len() < 32 {
        digits.extend(
            stable_id("", &format!("{seed}-{counter}"))
                .chars()
                .filter(|c| c.is_ascii_alphanumeric()),
        );
        counter += 1;
    }
    digits.truncate(32);
    format!(
        "{}-{}-{}-{}-{}",
        &digits[0..8],
        &digits[8..12],
        &digits[12..16],
        &digits[16..20],
        &digits[20..32]
    )
}
