//! Putting a picture into a note.
//!
//! A picture is markdown, `![name](path)`, on a paragraph of its own, which the
//! editor then shows as the picture itself and hands back as that markdown when
//! the cursor reaches it. This file builds that markdown around the cursor and
//! asks the system for the file.

use std::cell::RefCell;
use std::ops::Range;
use std::path::Path;

use aimer::ProviderHandle;
use aimer::image_picker::ImagePicker;
use aimer::input::{TextEditingController, TextEditingValue, TextSelection};
use aimer::Venus;

use crate::store::NoteStore;

thread_local! {
    /// The controller of the note being edited, so the toolbar, which is not part
    /// of the editor, can put something into the text.
    static EDITOR: RefCell<Option<TextEditingController>> = const { RefCell::new(None) };
}

/// Remembers `controller` as the one the toolbar inserts into.
pub fn register_editor(controller: &TextEditingController) {
    EDITOR.with(|editor| *editor.borrow_mut() = Some(controller.clone()));
}

/// The markdown of a picture: `![name](path)`.
///
/// A path with spaces or parentheses goes in angle brackets, which is how
/// markdown writes a destination that would otherwise end early, and the name
/// loses the brackets that would end its own.
pub fn picture_markdown(name: &str, path: &str) -> String {
    let name: String = name.chars().filter(|&character| character != '[' && character != ']').collect();
    let needs_brackets = path.chars().any(|character| character.is_whitespace() || character == '(' || character == ')');
    if needs_brackets {
        format!("![{name}](<{path}>)")
    } else {
        format!("![{name}]({path})")
    }
}

/// Puts `picture` into `text` in place of the selection, on a paragraph of its
/// own, and returns the new text and where the cursor goes: after the picture,
/// on the empty line below it, ready for more writing.
///
/// A blank line is added before and after only where there is not one already, so
/// putting a picture next to another picture, or at the start or end of the note,
/// does not pile up empty lines.
pub fn insert_picture(text: &str, selection: Range<usize>, picture: &str) -> (String, usize) {
    let start = selection.start.min(text.len());
    let end = selection.end.clamp(start, text.len());
    let (before, after) = (&text[..start], &text[end..]);

    let lead = match blank_lines_before(before) {
        _ if before.is_empty() => "",
        0 => "\n\n",
        1 => "\n",
        _ => "",
    };
    let trail = match blank_lines_after(after) {
        _ if after.is_empty() => "\n",
        0 => "\n\n",
        1 => "\n",
        _ => "",
    };
    let mut result = String::with_capacity(text.len() + picture.len() + 4);
    result.push_str(before);
    result.push_str(lead);
    result.push_str(picture);
    result.push_str(trail);
    let cursor = result.len();
    result.push_str(after);
    (result, cursor)
}

/// How many blank lines end `before`: 0 if it ends in text, 1 if it ends in a
/// line break, 2 or more if the last line is empty too.
fn blank_lines_before(before: &str) -> usize {
    before.chars().rev().take_while(|&character| character == '\n').count()
}

/// How many line breaks start `after`.
fn blank_lines_after(after: &str) -> usize {
    after.chars().take_while(|&character| character == '\n').count()
}

/// Asks for a picture with the system's picker and puts it into note `note` at
/// the cursor of the open editor.
///
/// Does nothing if the picker is dismissed. The picker is asynchronous, so this
/// returns at once and the note changes when a file is chosen.
pub fn pick_and_insert(store: ProviderHandle<NoteStore>, note: u64) {
    let Some(venus) = Venus::current() else {
        return;
    };
    venus.spawn(async move {
        let Ok(Some(file)) = ImagePicker::new().pick_one().await else {
            return;
        };
        let Some(path) = file.path().and_then(Path::to_str) else {
            return;
        };
        let name = Path::new(file.name()).file_stem().and_then(|stem| stem.to_str()).unwrap_or("image");
        let picture = picture_markdown(name, path);
        let Some(controller) = EDITOR.with(|editor| editor.borrow().clone()) else {
            return;
        };
        let value = controller.value();
        let selection = value.selection();
        let range = selection.anchor().min(selection.focus())..selection.anchor().max(selection.focus());
        let (text, cursor) = insert_picture(&value.text(), range, &picture);
        controller.set_value(TextEditingValue::new(text.clone(), TextSelection::new(cursor, cursor), None));
        // A change made in code is not reported by the field, so the note is told.
        store.update(|store| store.set_body(note, text));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const PICTURE: &str = "![cat](/tmp/cat.png)";

    #[test]
    fn a_picture_is_markdown_with_the_name_and_the_path() {
        assert_eq!(picture_markdown("cat", "/tmp/cat.png"), PICTURE);
    }

    #[test]
    fn a_path_that_would_end_early_goes_in_angle_brackets() {
        assert_eq!(picture_markdown("my cat", "/tmp/my cat.png"), "![my cat](</tmp/my cat.png>)");
        assert_eq!(picture_markdown("a", "/tmp/a(1).png"), "![a](</tmp/a(1).png>)");
    }

    #[test]
    fn brackets_in_the_name_are_dropped() {
        assert_eq!(picture_markdown("a[1]", "/x.png"), "![a1](/x.png)");
    }

    #[test]
    fn a_picture_in_an_empty_note_stands_alone_with_the_cursor_below_it() {
        let (text, cursor) = insert_picture("", 0..0, PICTURE);

        assert_eq!(text, format!("{PICTURE}\n"));
        assert_eq!(cursor, text.len());
    }

    #[test]
    fn a_picture_after_a_line_of_text_gets_a_blank_line_on_each_side() {
        let note = "title\nsome words";
        let (text, cursor) = insert_picture(note, note.len()..note.len(), PICTURE);

        assert_eq!(text, format!("title\nsome words\n\n{PICTURE}\n"));
        assert_eq!(&text[cursor..], "");
    }

    #[test]
    fn a_picture_in_the_middle_of_a_line_splits_it_into_paragraphs() {
        let note = "before after";
        let (text, cursor) = insert_picture(note, 7..7, PICTURE);

        assert_eq!(text, format!("before \n\n{PICTURE}\n\nafter"));
        assert_eq!(&text[cursor..], "after", "the cursor is where the text goes on");
    }

    #[test]
    fn existing_blank_lines_are_not_doubled() {
        let note = "one\n\ntwo";
        let (text, _) = insert_picture(note, 5..5, PICTURE);

        assert_eq!(text, format!("one\n\n{PICTURE}\n\ntwo"));
    }

    #[test]
    fn a_picture_on_its_own_line_after_a_single_break_gets_only_the_missing_one() {
        let note = "one\ntwo";
        let (text, _) = insert_picture(note, 4..4, PICTURE);

        assert_eq!(text, format!("one\n\n{PICTURE}\n\ntwo"));
    }

    #[test]
    fn a_selection_is_replaced_by_the_picture() {
        let note = "keep REPLACE keep";
        let (text, _) = insert_picture(note, 5..12, PICTURE);

        assert_eq!(text, format!("keep \n\n{PICTURE}\n\n keep"));
    }

    #[test]
    fn two_pictures_in_a_row_are_separate_paragraphs() {
        let (first, cursor) = insert_picture("", 0..0, PICTURE);
        let (second, _) = insert_picture(&first, cursor..cursor, "![dog](/tmp/dog.png)");

        assert_eq!(second, format!("{PICTURE}\n\n![dog](/tmp/dog.png)\n"));
    }

    #[test]
    fn a_selection_past_the_end_is_clamped() {
        let (text, _) = insert_picture("abc", 10..20, PICTURE);

        assert_eq!(text, format!("abc\n\n{PICTURE}\n"));
    }
}
