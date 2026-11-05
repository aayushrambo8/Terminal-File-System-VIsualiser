use std::path::Path;
use std::process::Command;

use super::detect_opener;

pub fn launch_file(path: &Path) -> std::io::Result<()> {
    let mut command_parts = detect_opener(path);
    if command_parts.is_empty() {
        command_parts.push("xdg-open".to_string());
    }

    let program = command_parts.remove(0);
    let mut command = Command::new(program);
    command.args(command_parts).arg(path);
    command.status()?;
    Ok(())
}
