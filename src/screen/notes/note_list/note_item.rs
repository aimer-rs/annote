use crate::components::icon::icon;
use crate::components::pad::pad;
use crate::models::note_row::NoteRow;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::decoration::Decoration;
use aimer::input::button::Button;
use aimer::share::ShareRef;
use aimer::style::{BorderRadius, FontWeight, LayoutSpacing, TextOverflow, TextStyle, Theme};
use aimer::{
    BoxAlignment, BuildContext, Color, Column, Container, Dimension, Expanded, Icons,
    ProviderContext, ProviderHandle, Row, SizedBox, StatelessWidget, Text, Widget,
};

/// One note in the list: its title, date and a line of preview.
#[derive(Clone, StatelessWidget)]
pub struct NoteItem {
    row: ShareRef<NoteRow>,
}

impl NoteItem {
    pub fn new(row: ShareRef<NoteRow>) -> Self {
        Self { row }
    }
}

impl StatelessWidget for NoteItem {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let id = self.row.id;
        let selected = ctx.select::<NoteStore, bool>(move |s| s.selected_note == Some(id));
        let store = ProviderHandle::<NoteStore>::of(ctx);
        let row = self.row.clone();

        Button::new()
            .no_decor()
            .on_press(move || store.update(|s| s.select_note(id)))
            .build(move |state| {
                let background = if selected {
                    theme.accent
                } else if state.is_hovered {
                    theme.hover
                } else {
                    Color::Transparent
                };
                let title = if selected {
                    theme.on_accent
                } else {
                    theme.text
                };
                let subtle = if selected {
                    theme.on_accent.with_alpha(0.8)
                } else {
                    theme.muted
                };
                // Each text is a handle to one field of the row, so nothing is copied.
                let row_title = row.clone().project(|r: &NoteRow| r.title.as_str());
                let row_date = row.clone().project(|r: &NoteRow| r.date.as_str());
                let row_preview = row.clone().project(|r: &NoteRow| r.preview.as_str());
                Container::new()
                    .width(Dimension::Percent(100.0))
                    .padding(pad(10, 14, 10, 14))
                    .decoration([BorderRadius::all(10).notype(), background.notype()])
                    .child(
                        Column::new().children([
                            Text::new(row_title)
                                .text_style(
                                    TextStyle::new()
                                        .font_size(15)
                                        .font_weight(FontWeight::Bold)
                                        .text_overflow(TextOverflow::Ellipsis)
                                        .color(title),
                                )
                                .boxed(),
                            SizedBox::new().height(3).boxed(),
                            Row::new()
                                .vertical_alignment(BoxAlignment::Center)
                                .gaps(LayoutSpacing::all(8))
                                .children([
                                    if row.pinned {
                                        icon(Icons::PIN, 12, subtle).boxed()
                                    } else {
                                        SizedBox::new().width(12).boxed()
                                    },
                                    Text::new(row_date)
                                        .text_style(TextStyle::new().font_size(13).color(title))
                                        .boxed(),
                                    Expanded::new()
                                        .child(
                                            Text::new(row_preview).text_style(
                                                TextStyle::new()
                                                    .font_size(13)
                                                    .color(subtle)
                                                    .text_overflow(TextOverflow::Ellipsis),
                                            ),
                                        )
                                        .boxed(),
                                ])
                                .boxed(),
                        ]),
                    )
            })
    }
}
