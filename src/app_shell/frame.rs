use crate::screen::notes::NotesScreen;
use crate::theme::{NATIVE_BACKDROP, NoteTheme};
use aimer::style::{BoxDecoration, Theme};
use aimer::*;

/// The window's backdrop and rounded corners, around the screen.
#[derive(Clone, StatelessWidget)]
pub struct Frame;

impl StatelessWidget for Frame {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        // The window's blur is what the sidebar tints; the other two panels cover
        // it with an opaque fill of their own.
        // The shell is transparent over the blur and rounds the window's corners,
        // as in Jaime; without a blur it paints the editor's color instead.
        let shell_fill = if NATIVE_BACKDROP {
            Color::Transparent
        } else {
            theme.editor
        };
        WindowBackdrop::new()
            .material(WindowMaterial::Sidebar)
            .enabled(NATIVE_BACKDROP)
            .child(
                Container::new()
                    .box_decoration(
                        BoxDecoration::new()
                            .background_color(shell_fill)
                            .border_radius(24),
                    )
                    .child(NotesScreen),
            )
    }
}
