use std::path::Path;

use tfsv::filesystem::{ascii_safe_display, validate_ascii_output};

#[test]
fn ascii_safe_display_escapes_non_ascii() {
    let path = Path::new("report-✓.txt");
    let safe = ascii_safe_display(path);
    assert!(validate_ascii_output(&safe));
    assert!(safe.contains("report"));
    assert!(safe.contains("\\x"));
}

#[test]
fn ascii_outputs_are_only_visible_ascii() {
    let text = "safe-name_123";
    assert!(validate_ascii_output(text));
}
