use aimer::{Color, Theme};

/// Whether the window blurs what is behind it, which the sidebar then shows.
/// macOS only, as in the Jaime showcase.
pub const NATIVE_BACKDROP: bool = cfg!(target_os = "macos");

/// Colours of the app. Derives `Theme` so `AnimatedTheme` can fade between the
/// light and dark variants.
#[derive(Clone, Copy, Debug, PartialEq, Theme)]
pub struct NoteTheme {
    /// Sidebar background.
    pub sidebar: Color,
    /// Note list background.
    pub list: Color,
    /// Editor background.
    pub editor: Color,
    pub text: Color,
    pub muted: Color,
    pub divider: Color,
    /// Accent used for the selected row and the folder icons.
    pub accent: Color,
    pub on_accent: Color,
    /// Background of a hovered row or button.
    pub hover: Color,
    pub field: Color,
}

impl NoteTheme {
    /// The sidebar's fill: a tint over the window's blur where there is one.
    pub fn sidebar_fill(&self) -> Color {
        if NATIVE_BACKDROP { self.sidebar.with_alpha(0.66) } else { self.sidebar }
    }

    pub fn dark() -> Self {
        Self {
            sidebar: Color::Rgb(33, 38, 46),
            list: Color::Rgb(29, 30, 33),
            editor: Color::Rgb(30, 30, 30),
            text: Color::Rgb(235, 235, 240),
            muted: Color::Rgb(142, 146, 154),
            divider: Color::Rgb(50, 52, 56),
            accent: Color::Rgb(164, 137, 58),
            on_accent: Color::Rgb(255, 255, 255),
            hover: Color::Rgb(255, 255, 255).with_alpha(0.07),
            field: Color::Rgb(255, 255, 255).with_alpha(0.08),
        }
    }

    pub fn light() -> Self {
        Self {
            sidebar: Color::Rgb(240, 238, 232),
            list: Color::Rgb(250, 250, 248),
            editor: Color::Rgb(255, 255, 255),
            text: Color::Rgb(30, 30, 32),
            muted: Color::Rgb(120, 122, 128),
            divider: Color::Rgb(222, 220, 214),
            accent: Color::Rgb(214, 170, 52),
            on_accent: Color::Rgb(30, 30, 32),
            hover: Color::Rgb(0, 0, 0).with_alpha(0.05),
            field: Color::Rgb(0, 0, 0).with_alpha(0.06),
        }
    }
}
