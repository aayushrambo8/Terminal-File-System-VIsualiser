use std::path::Path;

pub fn validate_ascii_output(value: &str) -> bool {
    value.bytes().all(|b| (0x20..=0x7E).contains(&b))
}

pub fn ascii_safe_display(path: &Path) -> String {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>");
    let safe = file_name
        .chars()
        .map(|ch| {
            if ch.is_control() {
                format!("\\u{{{:X}}}", ch as u32)
            } else {
                ch.to_string()
            }
        })
        .collect::<String>();
    let mut chars = safe.chars();
    let truncated = chars.by_ref().take(64).collect::<String>();
    if chars.next().is_some() {
        format!("{}...", truncated)
    } else {
        safe
    }
}
