use crate::models::date::format_date;
use crate::models::folder::Folder;
use crate::models::note::Note;
use crate::models::note_row::NoteRow;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct NoteStore {
    pub folders: Vec<Folder>,
    pub notes: Vec<Note>,
    /// `None` shows the notes of every folder.
    pub selected_folder: Option<u64>,
    pub selected_note: Option<u64>,
    pub search: String,
    next_id: u64,
}

impl Default for NoteStore {
    fn default() -> Self {
        let now = now();
        let folders = vec![
            Folder { id: 1, name: "Notes".into() },
            Folder { id: 2, name: "Work".into() },
            Folder { id: 3, name: "Journal".into() },
        ];
        let sample = |id, folder, body: &str, pinned, ago: u64| Note {
            id,
            folder,
            body: body.to_owned(),
            pinned,
            updated: now - ago * 3600,
        };
        let notes = vec![
            sample(
                1,
                1,
                "Welcome to Annote\nA simple note app built with Aimer. Start typing to edit this note, use the toolbar to create, pin or delete notes.",
                true,
                1,
            ),
            sample(2, 1, "Shopping list\n- Milk\n- Coffee beans\n- Bread", false, 5),
            sample(3, 2, "Widget delivery plan\nThis is the implementation backlog for missing widgets.\n\nScope\nControls, forms, overlays and navigation.", false, 30),
            sample(4, 3, "Spending recording\nAugust: groceries, transport, books.", true, 90),
            sample(5, 3, "Daily dish\nPasta with tomato and basil.", false, 200),
        ];
        Self { folders, notes, selected_folder: None, selected_note: Some(1), search: String::new(), next_id: 100 }
    }
}

impl NoteStore {
    pub fn note(&self, id: u64) -> Option<&Note> {
        self.notes.iter().find(|n| n.id == id)
    }

    pub fn folder_count(&self, folder: Option<u64>) -> usize {
        self.notes.iter().filter(|n| folder.is_none_or(|f| n.folder == f)).count()
    }

    /// Notes of the selected folder matching the search, pinned first and then
    /// most recently edited first.
    fn visible(&self) -> Vec<&Note> {
        let query = self.search.trim().to_lowercase();
        let mut notes: Vec<&Note> = self
            .notes
            .iter()
            .filter(|n| self.selected_folder.is_none_or(|f| n.folder == f))
            .filter(|n| query.is_empty() || n.body.to_lowercase().contains(&query))
            .collect();
        notes.sort_by(|a, b| b.pinned.cmp(&a.pinned).then(b.updated.cmp(&a.updated)));
        notes
    }

    pub fn rows(&self) -> Vec<NoteRow> {
        self.visible()
            .into_iter()
            .map(|n| NoteRow {
                id: n.id,
                title: n.title(),
                preview: n.preview(),
                date: format_date(n.updated),
                pinned: n.pinned,
            })
            .collect()
    }

    pub fn select_folder(&mut self, folder: Option<u64>) {
        self.selected_folder = folder;
        self.selected_note = self.visible().first().map(|n| n.id);
    }

    pub fn select_note(&mut self, id: u64) {
        self.selected_note = Some(id);
    }

    pub fn set_search(&mut self, search: String) {
        self.search = search;
    }

    /// Creates an empty note in the selected folder (or the first one) and opens it.
    pub fn new_note(&mut self) {
        let folder = self.selected_folder.or_else(|| self.folders.first().map(|f| f.id)).unwrap_or(1);
        let id = self.alloc_id();
        self.search.clear();
        self.notes.push(Note { id, folder, body: String::new(), pinned: false, updated: now() });
        self.selected_note = Some(id);
    }

    pub fn new_folder(&mut self) {
        let id = self.alloc_id();
        let name = format!("Folder {}", self.folders.len() + 1);
        self.folders.push(Folder { id, name });
        self.select_folder(Some(id));
    }

    pub fn set_body(&mut self, id: u64, body: String) {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            if note.body != body {
                note.body = body;
                note.updated = now();
            }
        }
    }

    pub fn toggle_pin(&mut self, id: u64) {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.pinned = !note.pinned;
        }
    }

    /// Deletes a note and selects the one that took its place in the list.
    pub fn delete_note(&mut self, id: u64) {
        let order: Vec<u64> = self.visible().iter().map(|n| n.id).collect();
        self.notes.retain(|n| n.id != id);
        let at = order.iter().position(|&n| n == id).unwrap_or(0);
        let next: Vec<u64> = order.into_iter().filter(|&n| n != id).collect();
        self.selected_note = next.get(at).or(next.last()).copied();
    }

    fn alloc_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_selects_neighbour() {
        let mut store = NoteStore::default();
        let first = store.rows()[0].id;
        store.select_note(first);
        store.delete_note(first);
        assert_ne!(store.selected_note, Some(first));
        assert!(store.selected_note.is_some());
    }

    #[test]
    fn search_filters() {
        let mut store = NoteStore::default();
        store.set_search("coffee".into());
        assert_eq!(store.rows().len(), 1);
    }
}
