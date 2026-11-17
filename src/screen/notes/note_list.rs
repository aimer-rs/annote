mod note_item;

use self::note_item::NoteItem;
use crate::components::pad::pad;
use crate::models::note_row::NoteRow;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::share::{ShareRef, Shared};
use aimer::style::{FontWeight, LayoutSpacing, TextStyle, Theme};
use aimer::{
    BuildContext, Column, Container, Expanded, ProviderContext, Scrollable, StatelessWidget, Text, Widget,
};

/// The middle column: the notes of the selected folder.
#[derive(Clone, StatelessWidget)]
pub struct NoteList;

impl StatelessWidget for NoteList {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let rows = ctx.select::<NoteStore, Vec<NoteRow>>(|s| s.rows());
        let (name, count) = ctx.select::<NoteStore, (String, usize)>(|s| {
            let name = match s.selected_folder {
                None => "All Notes".to_owned(),
                Some(id) => s
                    .folders
                    .iter()
                    .find(|f| f.id == id)
                    .map(|f| f.name.clone())
                    .unwrap_or_default(),
            };
            (name, s.folder_count(s.selected_folder))
        });

        // The rows move into the items, so no row is copied on the way.
        let (pinned, others): (Vec<_>, Vec<_>) = rows.into_iter().partition(|r| r.pinned);
        let is_empty = pinned.is_empty() && others.is_empty();

        let mut items: Vec<_> = Vec::new();
        if !pinned.is_empty() {
            items.push(section_header("Pinned", theme).boxed());
            items.extend(pinned.into_iter().map(|r| note_item(r).boxed()));
        }
        if !others.is_empty() {
            if !items.is_empty() {
                items.push(section_header("Notes", theme).boxed());
            }
            items.extend(others.into_iter().map(|r| note_item(r).boxed()));
        }
        if is_empty {
            items.push(
                Container::new()
                    .padding(LayoutSpacing::all(24))
                    .child(
                        Text::new("No Notes")
                            .text_style(TextStyle::new().font_size(15).color(theme.muted)),
                    )
                    .boxed(),
            );
        }

        Container::new().color(theme.list).child(
            Column::new().children([
                Container::new()
                    .padding(pad(16, 16, 10, 16))
                    .child(
                        Column::new().children([
                            Text::new(name)
                                .text_style(
                                    TextStyle::new()
                                        .font_size(16)
                                        .font_weight(FontWeight::Bold)
                                        .color(theme.text),
                                )
                                .boxed(),
                            Text::new(format!("{count} notes"))
                                .text_style(TextStyle::new().font_size(12).color(theme.muted))
                                .boxed(),
                        ]),
                    )
                    .boxed(),
                Expanded::new()
                    .child(
                        Scrollable::new().vertical_scroll_bar(None).child(
                            Container::new()
                                .padding(pad(0, 8, 12, 8))
                                .child(Column::new().children(items)),
                        ),
                    )
                    .boxed(),
            ]),
        )
    }
}

fn note_item(row: NoteRow) -> NoteItem {
    NoteItem::new(ShareRef::from(Shared::new(row)))
}

fn section_header(label: &'static str, theme: NoteTheme) -> impl Widget {
    Container::new().padding(pad(14, 8, 6, 8)).child(
        Text::new(label).text_style(
            TextStyle::new()
                .font_size(17)
                .font_weight(FontWeight::Bold)
                .color(theme.text),
        ),
    )
}
