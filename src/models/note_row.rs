/// What the list draws for one note.
#[derive(Clone, Debug, PartialEq)]
pub struct NoteRow {
    pub id: u64,
    pub title: String,
    pub preview: String,
    pub date: String,
    pub pinned: bool,
}
