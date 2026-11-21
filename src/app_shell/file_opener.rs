use crate::store::NoteStore;
use aimer::events::file_open::{self, FileGuard};
use aimer::{
    AnyWidget, BuildContext, ChildBuilder, ProviderHandle, State, StateView, StatefulWidget, Venus, Widget,
};
use std::path::{Path, PathBuf};

/// What a file may be called to be opened as a note.
const NOTE_EXTENSIONS: [&str; 3] = ["md", "markdown", "txt"];

/// Opens the files the app is asked to open as notes: a markdown file dropped on
/// the app's icon, chosen in "Open With", or double-clicked. The kinds of file
/// the system offers the app are declared under `[[documents]]` in `Aimer.toml`.
///
/// It draws only its child. The file that *launched* the app arrived before there
/// was a tree to take it, so it is claimed here as the first screen is built.
#[derive(Clone, StatefulWidget)]
pub struct FileOpener {
    child: ChildBuilder,
}

impl FileOpener {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self { child: ChildBuilder::from_widget(child) }
    }
}

struct FileOpenerState {
    child: ChildBuilder,
    /// Keeps the handler registered for as long as the app has this screen.
    registration: Option<FileGuard>,
}

impl State<FileOpener> for FileOpenerState {
    fn create(widget: FileOpener) -> Self {
        Self { child: widget.child, registration: None }
    }

    fn init_state(&mut self) {
        let store = ProviderHandle::<NoteStore>::of(BuildContext::current().get());

        let handler_store = store.clone();
        self.registration = Some(file_open::register(move |paths: &[PathBuf]| open(&handler_store, paths)));

        // The store is not changed while the tree is still being built: the file that
        // launched the app is claimed once this frame is done.
        if let Some(venus) = Venus::current() {
            venus.spawn(async move {
                file_open::claim_waiting_files(|paths| open(&store, paths));
            });
        }
    }

    fn adopt_config_from(&mut self, new: Self) {
        self.child = new.child;
    }

    fn build(this: StateView<Self>, _ctx: &BuildContext) -> impl Widget {
        AnyWidget::from(this.child.clone().boxed())
    }
}

/// Opens each of `paths` that is a note as a note of its own, the last one open,
/// and returns whether any was. A path that is not a note, or cannot be read as
/// text, is left for whatever else may take it.
fn open(store: &ProviderHandle<NoteStore>, paths: &[PathBuf]) -> bool {
    let bodies: Vec<String> = paths
        .iter()
        .filter(|path| is_note(path))
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .collect();
    if bodies.is_empty() {
        return false;
    }
    store.update(|store| {
        for body in bodies {
            store.add_note(body);
        }
    });
    true
}

fn is_note(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| NOTE_EXTENSIONS.iter().any(|note| note.eq_ignore_ascii_case(extension)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notes_are_markdown_and_text_files() {
        assert!(is_note(Path::new("/a/b.md")));
        assert!(is_note(Path::new("/a/B.MARKDOWN")));
        assert!(is_note(Path::new("c.txt")));
        assert!(!is_note(Path::new("/a/b.rs")));
        assert!(!is_note(Path::new("/a/md")));
    }
}
