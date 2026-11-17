mod note_date;

use self::note_date::NoteDate;
use super::TOOLBAR_HEIGHT;
use crate::components::pad::pad;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::Color;
use aimer::input::TextArea;
use aimer::input::TextEditingController;
use aimer::input::input::{TextEmbedder, TextHighlighter};
use aimer::style::{BoxDecoration, TextAlign, TextStyle, Theme};
use aimer::{
    BuildContext, Column, Container, Key, ProviderHandle, Scrollable, SizedBox, State, StateView,
    StatefulWidget, Text, Widget,
};

/// How big a note may be, in bytes, before it is edited as plain text.
///
/// The text field lays out only the part of a long note that is on screen, and the
/// preview reads a note in pieces and reads again only the one that was typed in, so a
/// note of a few megabytes opens, scrolls and takes typing with the preview on: a debug
/// build was tried with a 2.3 MB note of headings, lists, code and links. This is a
/// safety net past that, not a measured limit. A field is also held to eight million
/// pixels of height, which a note of short lines reaches in fewer bytes.
const LARGE_NOTE_BYTES: usize = 4_000_000;

/// The date and the text of one note. The text is edited in place and the store is
/// told of every change. The widget is keyed by the note, so each note gets a
/// state of its own, made once when it opens.
#[derive(Clone, StatefulWidget)]
pub struct NoteBody {
    key: Option<Key>,
    id: u64,
    initial: String,
}

impl NoteBody {
    pub fn new(id: u64, initial: String) -> Self {
        Self { key: Some(Key::from(("note", id))), id, initial }
    }
}

struct NoteBodyState {
    id: u64,
    controller: TextEditingController,
    /// Made once: a new highlighter on every rebuild would restyle the whole note.
    /// `None` for a large note, which is edited as plain text.
    highlighter: Option<TextHighlighter>,
    /// Shows the tables and code blocks of the note as widgets. `None` as above.
    embedder: Option<TextEmbedder>,
}

impl NoteBodyState {
    fn is_large(&self) -> bool {
        self.highlighter.is_none()
    }
}

impl State<NoteBody> for NoteBodyState {
    fn create(widget: NoteBody) -> Self {
        // Decided once, when the note opens, so the editor never changes under the caret.
        let (highlighter, embedder) = if widget.initial.len() > LARGE_NOTE_BYTES {
            (None, None)
        } else {
            let (highlighter, embedder) = crate::markdown::highlighter_and_embedder();
            (Some(highlighter), Some(embedder))
        };
        Self {
            id: widget.id,
            controller: TextEditingController::with_text(widget.initial),
            highlighter,
            embedder,
        }
    }

    fn init_state(&mut self) {
        // The toolbar puts pictures into whatever note this editor holds.
        crate::images::register_editor(&self.controller);
    }

    fn adopt_config_from(&mut self, _new: Self) {
        // `create` runs again whenever the editor above is rebuilt, and its state is
        // thrown away here. The live state is the one on screen, and the one the
        // toolbar has to insert into.
        crate::images::register_editor(&self.controller);
    }

    fn build(this: StateView<Self>, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let store = ProviderHandle::<NoteStore>::of(ctx);
        let id = this.id;

        let mut area = TextArea::new()
            .controller(this.controller.clone())
            .hint("Start typing…")
            .decoration(Color::Transparent)
            .hint_style(TextStyle::new().font_size(16).color(theme.muted))
            .text_style(TextStyle::new().font_size(16).color(theme.text))
            .selection_color(theme.accent.with_alpha(0.4))
            .tab_spaces(4)
            .padding(pad(0, 40, 24, 40))
            .on_changed(move |text: String| store.update(|s| s.set_body(id, text)));
        if let Some(highlighter) = &this.highlighter {
            area = area.highlighter(highlighter.clone());
        }
        if let Some(embedder) = &this.embedder {
            area = area.embedder(embedder.clone());
        }

        let mut page = vec![
            SizedBox::new().height(TOOLBAR_HEIGHT).boxed(),
            NoteDate::new(id).boxed(),
        ];
        if this.is_large() {
            page.push(
                Container::new()
                    .padding(pad(0, 40, 8, 40))
                    .child(
                        Text::new("Large note: the formatting preview is off.")
                            .text_align(TextAlign::MidCenter)
                            .text_style(TextStyle::new().font_size(12).color(theme.muted)),
                    )
                    .boxed(),
            );
        }
        page.push(area.boxed());

        // The date and the area scroll together, under the floating toolbar, so a long
        // note passes beneath its glass. The area is as tall as its text and the pane
        // scrolls it, so a long note never scrolls inside the field itself.
        Scrollable::new().child(Column::new().children(page))
    }
}
