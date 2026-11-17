use crate::components::icon::icon;
use crate::components::pad::pad;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::decoration::Decoration;
use aimer::input::button::Button;
use aimer::share::ShareRef;
use aimer::style::{BorderRadius, LayoutSpacing, TextStyle, Theme};
use aimer::{
    BoxAlignment, BuildContext, Color, Container, Dimension, Expanded, Icons, ProviderContext, ProviderHandle,
    Row, StatelessWidget, Text, Widget,
};

/// One folder in the sidebar: its name and how many notes it holds.
#[derive(Clone, StatelessWidget)]
pub struct FolderRow {
    folder: Option<u64>,
    name: ShareRef<str>,
}

impl FolderRow {
    pub fn new(folder: Option<u64>, name: ShareRef<str>) -> Self {
        Self { folder, name }
    }
}

impl StatelessWidget for FolderRow {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let id = self.folder;
        let selected = ctx.select::<NoteStore, bool>(move |s| s.selected_folder == id);
        let count = ctx.select::<NoteStore, usize>(move |s| s.folder_count(id));
        let store = ProviderHandle::<NoteStore>::of(ctx);
        let name = self.name.clone();

        Button::new().no_decor().on_press(move || store.update(|s| s.select_folder(id))).build(
            move |state| {
                let background = if selected {
                    theme.accent
                } else if state.is_hovered {
                    theme.hover
                } else {
                    Color::Transparent
                };
                let foreground = if selected { theme.on_accent } else { theme.text };
                let count_color = if selected { theme.on_accent.with_alpha(0.8) } else { theme.muted };
                Container::new()
                    .width(Dimension::Percent(100.0))
                    .padding(pad(8, 10, 8, 10))
                    .decoration([BorderRadius::all(8).notype(), background.notype()])
                    .child(
                        Row::new().vertical_alignment(BoxAlignment::Center).gaps(LayoutSpacing::all(10)).children([
                            icon(
                                if selected || id.is_none() { Icons::FOLDER_OPEN } else { Icons::FOLDER },
                                20,
                                if selected { theme.on_accent } else { theme.accent },
                            )
                            .boxed(),
                            Expanded::new()
                                .child(
                                    Text::new(name.clone())
                                        .text_style(TextStyle::new().font_size(15).color(foreground)),
                                )
                                .boxed(),
                            Text::new(count.to_string())
                                .text_style(TextStyle::new().font_size(14).color(count_color))
                                .boxed(),
                        ]),
                    )
            },
        )
    }
}
