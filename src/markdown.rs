//! The note editor's live-preview markdown.
//!
//! The framework's markdown parser does the reading: what the editor shows as a
//! heading, list, quote or code block is what the viewer would render from the
//! same text. This file only says how a note looks, as a [`MarkdownTheme`], and
//! that a note's first line is its title.

use aimer::input::input::{SpanStyle, TextEmbedder, TextHighlighter};
use aimer::style::{FontWeight, TextDecoration, TextStyle};
use aimer::{Color, MarkdownLivePreview, MarkdownTheme};

/// Heading sizes for `#` to `######`.
const HEADING_SIZES: [u32; 6] = [28, 24, 20, 18, 16, 16];
/// The size of a note's first line, its title.
const TITLE_SIZE: u32 = 24;

const MUTED: Color = Color::Rgba(140, 142, 152, 255);
const LINK: Color = Color::Rgba(90, 160, 255, 255);
const ACCENT: Color = Color::Rgba(214, 170, 70, 255);
const CODE_BACKGROUND: Color = Color::Rgba(127, 127, 127, 70);
const TEXT: Color = Color::Rgba(232, 234, 240, 255);

/// The highlighter and the embedder of a note's text. They share one parse, so
/// make them together, once, and clone them.
///
/// The highlighter styles the text as it is typed; the embedder shows the tables
/// and code blocks the cursor is not in as the viewer's widgets.
pub fn highlighter_and_embedder() -> (TextHighlighter, TextEmbedder) {
    let preview = preview();
    (preview.highlighter(), preview.embedder())
}

/// The highlighter of a note's text. Make it once and clone it.
#[cfg(test)]
pub fn highlighter() -> TextHighlighter {
    preview().highlighter()
}

fn preview() -> MarkdownLivePreview {
    let title = SpanStyle::new().font_size(TITLE_SIZE).font_weight(FontWeight::Bold);
    MarkdownLivePreview::new(&theme()).title(title)
}

/// How a note looks. The editor's own text style stays the body style, so only
/// what differs from it is set here.
fn theme() -> MarkdownTheme {
    let heading = |size: u32| TextStyle::new().font_size(size).font_weight(FontWeight::Bold);
    let defaults = MarkdownTheme::default();
    // The embedded tables and code blocks are drawn by the viewer on a dark page.
    let body = defaults.body.color(TEXT);
    MarkdownTheme {
        body,
        headings: HEADING_SIZES.map(heading),
        code_block: defaults.code_block.color(TEXT),
        // A table is its lines and nothing else: the page shows through the cells.
        table_header_background: Color::Transparent,
        table_cell_background: Color::Transparent,
        keyword_color: Color::Rgba(255, 123, 114, 255),
        string_color: Color::Rgba(165, 214, 255, 255),
        number_color: Color::Rgba(121, 192, 255, 255),
        blockquote: defaults.blockquote.color(MUTED),
        inline_code: defaults.inline_code.background_color(CODE_BACKGROUND),
        link: SpanStyle::new().color(LINK).text_decoration(TextDecoration::Underline),
        code_background: CODE_BACKGROUND,
        icon_color: ACCENT,
        comment_color: MUTED,
        rule_color: Color::Rgba(140, 142, 152, 120),
        ..defaults
    }
}

#[cfg(test)]
mod tests {
    use aimer::input::input::HighlightContext;

    use super::*;

    fn size_at(text: &str, cursor: usize, at: usize) -> Option<u32> {
        preview()
            .ranges(text, &HighlightContext::new(text, cursor..cursor))
            .iter()
            .find(|range| range.range.contains(&at))
            .and_then(|range| range.style.font_size)
    }

    #[test]
    fn the_first_line_is_the_title() {
        let text = "My note\nbody";

        assert_eq!(size_at(text, text.len(), 2), Some(TITLE_SIZE));
        assert_eq!(size_at(text, 0, 2), Some(TITLE_SIZE), "and stays one under the cursor");
    }

    #[test]
    fn headings_take_the_sizes_of_the_note_theme() {
        let text = "title\n# One\n### Three\nbody";
        let end = text.len();

        assert_eq!(size_at(text, end, text.find("One").unwrap()), Some(HEADING_SIZES[0]));
        assert_eq!(size_at(text, end, text.find("Three").unwrap()), Some(HEADING_SIZES[2]));
    }

    #[test]
    fn the_editor_highlighter_is_the_one_a_text_area_takes() {
        assert!(!highlighter().highlight("title\n# One").is_empty());
    }
}
