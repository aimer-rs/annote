mod note_body;
mod toolbar;

use self::note_body::NoteBody;
use self::toolbar::Toolbar;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::style::{TextStyle, Theme};
use aimer::{
    BuildContext, Container, Dimension, Positioned, ProviderContext, ProviderHandle, SizedBox,
    Stack, StatelessWidget, Text, Widget,
};

/// How tall the floating toolbar is: 36 for the buttons, and 10 of room above and
/// below them. The note keeps this much clear at its top, so it starts under the bar.
const TOOLBAR_HEIGHT: u32 = 56;

/// The right column: the open note, with a toolbar of glass floating over it.
#[derive(Clone, StatelessWidget)]
pub struct Editor;

impl StatelessWidget for Editor {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        // Only which note is open is watched here, so typing does not rebuild the
        // editor. The text is read once, as the note opens, and not subscribed to.
        let open = ctx.select::<NoteStore, Option<u64>>(|s| {
            s.selected_note.filter(|id| s.note(*id).is_some())
        });
        let store = ProviderHandle::<NoteStore>::of(ctx);

        let body = match open {
            Some(id) => {
                let initial = store
                    .read()
                    .note(id)
                    .map(|n| n.body.clone())
                    .unwrap_or_default();
                NoteBody::new(id, initial).boxed()
            }
            None => Container::new().box_child(
                Text::new("No note selected")
                    .text_style(TextStyle::new().font_size(16).color(theme.muted)),
            ),
        };

        Container::new().color(theme.editor).child(
            Stack::new().children([
                SizedBox::new()
                    .width(Dimension::Percent(100.0))
                    .height(Dimension::Percent(100.0))
                    .box_child(body),
                // The toolbar floats over the note, which scrolls under it.
                Positioned::new()
                    .layer(1)
                    .top(0)
                    .left(0)
                    .right(0)
                    .box_child(Toolbar)
                    .boxed(),
            ]),
        )
    }
}
