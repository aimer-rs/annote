mod file_opener;
mod frame;

use self::file_opener::FileOpener;
use self::frame::Frame;
use crate::theme::NoteTheme;
use aimer::animation::curves;
use aimer::style::AnimatedTheme;
use aimer::*;
use std::time::Duration;

/// Picks the light or dark theme from the system, and fades between them.
#[derive(Clone, StatelessWidget)]
pub struct AppShell;

impl StatelessWidget for AppShell {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = match ctx.watch_platform_brightness() {
            Brightness::Light => NoteTheme::light(),
            Brightness::Dark => NoteTheme::dark(),
        };
        AnimatedTheme::new()
            .data(theme)
            .duration(Duration::from_millis(300))
            .curve(curves::EASE_IN_OUT)
            .child(FileOpener::new(Frame))
    }
}
