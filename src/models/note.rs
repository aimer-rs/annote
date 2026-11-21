#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub id: u64,
    pub folder: u64,
    /// The whole note. Like Apple Notes, its first line is its title.
    ///
    /// Shared: a provider copies the whole store when a snapshot of it is still held, and a
    /// note of megabytes is then one more handle and not one more copy.
    pub body: std::rc::Rc<String>,
    pub pinned: bool,
    /// Seconds since the Unix epoch.
    pub updated: u64,
}

impl Note {
    pub fn title(&self) -> String {
        let line = self.body.lines().find(|l| !l.trim().is_empty());
        line.map(|l| l.trim().trim_start_matches('#').trim().to_owned())
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| "New Note".to_owned())
    }

    /// The first line after the title, for the list.
    pub fn preview(&self) -> String {
        let mut lines = self.body.lines().filter(|l| !l.trim().is_empty());
        lines.next();
        lines.next().map(|l| truncate(l.trim(), 24)).unwrap_or_else(|| "No additional text".to_owned())
    }
}

/// Cuts `text` to `max` characters, ending in an ellipsis when it was longer.
fn truncate(text: &str, max: usize) -> String {
    match text.char_indices().nth(max) {
        Some((end, _)) => format!("{}…", text[..end].trim_end()),
        None => text.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_and_preview() {
        let note = Note { id: 1, folder: 1, body: std::rc::Rc::new("\n# Hello\n\nworld".to_owned()), pinned: false, updated: 0 };
        assert_eq!(note.title(), "Hello");
        assert_eq!(note.preview(), "world");
        let empty = Note { body: Default::default(), ..note };
        assert_eq!(empty.title(), "New Note");
    }
}
