mod app_shell;
mod components;
mod images;
mod markdown;
mod models;
mod screen;
mod store;
mod theme;

use crate::app_shell::AppShell;
use crate::store::NoteStore;
use aimer::native::macos_windowing::MacosWindowing;
use aimer::quiver::window_attr::WindowAttr;
use aimer::*;

// this is the entry point of the app
#[aimer::main]
fn main() {
    AimerApp::new()
        .window(
            WindowAttr::new()
                .title("Annote")
                .inner_size(1100, 720)
                .min_inner_size(760, 480)
                .remember("annote-main"),
        )
        .setup(|| {
            // The title bar is drawn over the page, so the sidebar's material runs to the top.
            MacosWindowing::new()
                .titlebar_transparent(true)
                .title_hidden(true)
                .fullsize_content_view(true)
                .traffic_light_position(16.0, 14.0)
                .has_shadow(true)
                .accepts_first_mouse(true)
                .install();
        })
        .child(Provider::new().create(NoteStore::default).child(AppShell))
        .run()
}
