use crate::filesystem::FileEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortMode {
    Name,
    Type,
    Size,
}

pub fn sort_entries(entries: &mut [FileEntry], mode: SortMode) {
    match mode {
        SortMode::Name => entries.sort_by(|a, b| a.name.to_ascii_lowercase().cmp(&b.name.to_ascii_lowercase())),
        SortMode::Type => entries.sort_by(|a, b| a.kind.indicator().cmp(b.kind.indicator())),
        SortMode::Size => entries.sort_by(|a, b| b.size.cmp(&a.size)),
    }
}
