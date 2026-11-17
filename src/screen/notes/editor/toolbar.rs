mod search_box;

use self::search_box::SearchBox;
use super::TOOLBAR_HEIGHT;
use crate::components::icon_button::IconButton;
use crate::components::pad::pad;
use crate::store::NoteStore;
use aimer::style::LayoutSpacing;
use aimer::{
    BoxAlignment, BuildContext, Container, Expanded, Icons, ProviderContext, ProviderHandle, Row, SizedBox,
    StatelessWidget, Widget,
};

/// The buttons above the note, and the search field.
#[derive(Clone, StatelessWidget)]
pub struct Toolbar;

impl StatelessWidget for Toolbar {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let store = ProviderHandle::<NoteStore>::of(ctx);
        let selected = ctx.select::<NoteStore, Option<u64>>(|s| s.selected_note);
        let pinned = ctx.select::<NoteStore, bool>(|s| s.selected_note.and_then(|id| s.note(id)).is_some_and(|n| n.pinned));
        let (new_store, pin_store, delete_store, picture_store) =
            (store.clone(), store.clone(), store.clone(), store.clone());

        // A fixed height: floating over the page, the bar would otherwise be as tall as
        // the page, and its row would centre the buttons in it.
        Container::new().height(TOOLBAR_HEIGHT).padding(pad(10, 12, 10, 12)).child(
            Row::new().vertical_alignment(BoxAlignment::Center).gaps(LayoutSpacing::all(6)).children([
                IconButton::new(Icons::SQUARE_PEN, move || new_store.update(|s| s.new_note())).glass().boxed(),
                IconButton::new(if pinned { Icons::PIN_OFF } else { Icons::PIN }, move || {
                    if let Some(id) = selected {
                        pin_store.update(|s| s.toggle_pin(id));
                    }
                })
                .glass()
                .boxed(),
                IconButton::new(Icons::TRASH, move || {
                    if let Some(id) = selected {
                        delete_store.update(|s| s.delete_note(id));
                    }
                })
                .glass()
                .boxed(),
                IconButton::new(Icons::IMAGE_PLUS, move || {
                    if let Some(id) = selected {
                        crate::images::pick_and_insert(picture_store.clone(), id);
                    }
                })
                .glass()
                .boxed(),
                Expanded::new().child(SizedBox::new()).boxed(),
                SearchBox.boxed(),
            ]),
        )
    }
}
