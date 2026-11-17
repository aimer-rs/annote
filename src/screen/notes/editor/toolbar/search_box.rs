use crate::components::glass::glass;
use crate::components::icon::icon;
use crate::components::pad::pad;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::input::TextEditingController;
use aimer::input::input::TextField;
use aimer::style::{BoxDecoration, LayoutSpacing, TextStyle, Theme};
use aimer::{
    BoxAlignment, BuildContext, Container, Expanded, Icons, ProviderHandle, Row, State,
    StateView, StatefulWidget, Widget,
};

/// The search field at the right of the toolbar. It filters the note list.
#[derive(Clone, StatefulWidget)]
pub struct SearchBox;

struct SearchBoxState {
    controller: TextEditingController,
}

impl State<SearchBox> for SearchBoxState {
    fn create(_: SearchBox) -> Self {
        Self { controller: TextEditingController::new() }
    }

    fn build(this: StateView<Self>, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let store = ProviderHandle::<NoteStore>::of(ctx);
        Container::new()
            .width(220)
            .height(34)
            .decoration(glass(theme))
            .padding(pad(0, 10, 0, 10))
            .child(
                Row::new().vertical_alignment(BoxAlignment::Center).gaps(LayoutSpacing::all(6)).children([
                    icon(Icons::SEARCH, 16, theme.muted).boxed(),
                    Expanded::new()
                        .child(
                            TextField::new()
                                .controller(this.controller.clone())
                                .hint("Search")
                                .hint_style(TextStyle::new().font_size(14).color(theme.muted))
                                .text_style(TextStyle::new().font_size(14).color(theme.text))
                                .decoration(BoxDecoration::new())
                                .padding(LayoutSpacing::vertical(7))
                                .on_changed(move |text: String| store.update(|s| s.set_search(text))),
                        )
                        .boxed(),
                ]),
            )
    }
}
