/// Copy a regex pattern and ensure that it starts with the `^` anchor and ends with the `$` anchor.
pub fn ensure_regex_anchors(regex: &str) -> String {
    let start_anchor_present = regex.starts_with('^');
    let end_anchor_present = regex.ends_with('$');

    match (start_anchor_present, end_anchor_present) {
        (true, true) => regex.to_string(),
        (true, false) => format!("{}$", regex),
        (false, true) => format!("^{}", regex),
        (false, false) => format!("^{}$", regex),
    }
}

/// Check if the given string is a simple identifier. A simple identifier is an identifier that
/// contains only lower case ASCII alphabetic characters, digits, and underscores, and starts with
/// an alphabetic character.
///
/// A simple identifier can be used as a part of a field name in a struct, without needing to be
/// formatted in any way.
pub fn is_simple_ident(ident: &str) -> bool {
    let mut chars = ident.chars();

    (match chars.next() {
        None => false,
        Some(c) => c.is_ascii_lowercase(),
    }) && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_regex_anchors() {
        assert_eq!(ensure_regex_anchors("abc"), "^abc$");
        assert_eq!(ensure_regex_anchors("^abc$"), "^abc$");
        assert_eq!(ensure_regex_anchors("abc$"), "^abc$");
        assert_eq!(ensure_regex_anchors("^abc"), "^abc$");
        assert_eq!(ensure_regex_anchors(""), "^$");
    }

    #[test]
    fn test_is_simple_ident() {
        assert!(is_simple_ident("abc"));
        assert!(is_simple_ident("abc123"));
        assert!(is_simple_ident("abc_def"));
        assert!(is_simple_ident("a"));
        assert!(!is_simple_ident(""));
        assert!(!is_simple_ident("123"));
        assert!(!is_simple_ident("ABC"));
        assert!(!is_simple_ident("abc!"));
        assert!(!is_simple_ident("abc def"));
        assert!(!is_simple_ident("_abc"))
    }
}
