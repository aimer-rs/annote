mod folder_row;

use self::folder_row::FolderRow;
use crate::components::icon_button::IconButton;
use crate::components::pad::pad;
use crate::models::folder::Folder;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::share::ShareRef;
use aimer::style::{FontWeight, LayoutSpacing, TextStyle, Theme};
use aimer::{
    BoxAlignment, BuildContext, Column, Container, Expanded, Icons, ProviderContext, ProviderHandle, Row,
    Scrollable, SizedBox, StatelessWidget, Text, Widget,
};

/// The folder column on the left.
#[derive(Clone, StatelessWidget)]
pub struct Sidebar;

impl StatelessWidget for Sidebar {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let folders = ctx.select::<NoteStore, Vec<Folder>>(|s| s.folders.clone());
        let store = ProviderHandle::<NoteStore>::of(ctx);

        // The names move into shared handles once, so the rows never copy them again.
        let mut rows = vec![FolderRow::new(None, ShareRef::from_static("All Notes")).boxed()];
        rows.extend(folders.into_iter().map(|f| FolderRow::new(Some(f.id), ShareRef::from(f.name)).boxed()));

        Container::new().color(theme.sidebar_fill()).child(
            Column::new().children([
                // Leaves room for the traffic lights over the transparent title bar.
                SizedBox::new().height(44).boxed(),
                Container::new()
                    .padding(pad(0, 12, 0, 12))
                    .child(
                        Row::new().vertical_alignment(BoxAlignment::Center).children([
                            Expanded::new()
                                .child(
                                    Text::new("Folders").text_style(
                                        TextStyle::new()
                                            .font_size(13)
                                            .font_weight(FontWeight::Bold)
                                            .color(theme.muted),
                                    ),
                                )
                                .boxed(),
                            IconButton::new(Icons::FOLDER_PLUS, move || store.update(|s| s.new_folder()))
                                .size(32.0)
                                .boxed(),
                        ]),
                    )
                    .boxed(),
                Expanded::new()
                    .child(
                        Scrollable::new().vertical_scroll_bar(None).child(
                            Container::new()
                                .padding(pad(0, 10, 0, 10))
                                .child(Column::new().gaps(LayoutSpacing::all(2)).children(rows)),
                        ),
                    )
                    .boxed(),
            ]),
        )
    }
}
