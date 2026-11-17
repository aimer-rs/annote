mod editor;
mod note_list;
mod sidebar;

use self::editor::Editor;
use self::note_list::NoteList;
use self::sidebar::Sidebar;
use crate::components::resizable_panel::resizable_panel;
use crate::theme::NoteTheme;
use aimer::style::Theme;
use aimer::*;

const SIDEBAR_WIDTH: f32 = 230.0;
const SIDEBAR_MIN: f32 = 170.0;
const SIDEBAR_MAX: f32 = 400.0;

const LIST_WIDTH: f32 = 300.0;
const LIST_MIN: f32 = 220.0;
const LIST_MAX: f32 = 520.0;

/// The notes screen: folders, the notes of a folder, and the open note.
#[derive(Clone, StatelessWidget)]
pub struct NotesScreen;

impl StatelessWidget for NotesScreen {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        Row::new().children([
            resizable_panel(Sidebar, SIDEBAR_WIDTH, SIDEBAR_MIN, SIDEBAR_MAX, theme.divider).boxed(),
            resizable_panel(NoteList, LIST_WIDTH, LIST_MIN, LIST_MAX, theme.divider).boxed(),
            Expanded::new().child(Editor).boxed(),
        ])
    }
}
