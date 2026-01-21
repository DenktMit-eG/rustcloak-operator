use convert_case::{Case, Casing};

/// Convert a name to K8s-compatible kebab-case
pub fn to_k8s_name(name: &str) -> String {
    let name = name.to_case(Case::Kebab);
    sanitize_k8s_name(&name)
}

/// Sanitize name for K8s: alphanumeric start/end, max 253 chars
fn sanitize_k8s_name(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.')
        .collect();
    let sanitized = sanitized.trim_start_matches(|c: char| !c.is_ascii_alphanumeric());
    let sanitized = sanitized.trim_end_matches(|c: char| !c.is_ascii_alphanumeric());
    sanitized.chars().take(253).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_k8s_name() {
        assert_eq!(to_k8s_name("MyRealm"), "my-realm");
        assert_eq!(to_k8s_name("my-app"), "my-app");
        assert_eq!(to_k8s_name("My App Client"), "my-app-client");
        assert_eq!(to_k8s_name("admin-cli"), "admin-cli");
    }

    #[test]
    fn test_sanitize_special_chars() {
        // Special characters are stripped, underscores are converted to hyphens via case conversion
        assert_eq!(to_k8s_name("my_app@client!"), "my-appclient");
        assert_eq!(to_k8s_name("--my-app--"), "my-app");
    }
}
