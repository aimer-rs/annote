use crate::models::date::format_long_date;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::style::{LayoutSpacing, TextAlign, TextStyle, Theme};
use aimer::{BuildContext, Container, ProviderContext, StatelessWidget, Text, Widget};

/// When a note was last edited. It reads the date itself, so typing rebuilds this
/// one line and not the note around it.
#[derive(Clone, StatelessWidget)]
pub struct NoteDate {
    id: u64,
}

impl NoteDate {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl StatelessWidget for NoteDate {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let id = self.id;
        let updated = ctx.select::<NoteStore, u64>(move |s| s.note(id).map_or(0, |n| n.updated));
        Container::new().padding(LayoutSpacing::new().bottom(8)).child(
            Text::new(format_long_date(updated))
                .text_align(TextAlign::MidCenter)
                .text_style(TextStyle::new().font_size(12).color(theme.muted)),
        )
    }
}
