use crate::components::glass::glass;
use crate::components::icon::icon;
use crate::theme::NoteTheme;
use aimer::animation::{Animatable, ImplicitAnimatedBuilder};
use aimer::decoration::Decoration;
use aimer::input::button::{Button, ButtonVisualState};
use aimer::input::callback::VoidCallback;
use aimer::style::{BorderRadius, Theme, curves};
use aimer::{
    Align, Alignment, BuildContext, Color, Container, Scalable, StatelessWidget, StaticSvg, Widget,
};
use std::time::Duration;

/// How long the button takes to settle into a new state.
const DURATION: Duration = Duration::from_millis(300);
/// How much larger a floating button is while it is pressed.
const PRESSED_SCALE: f32 = 1.15;

/// What the button looks like in one state; it is animated between states.
#[derive(Copy, Clone, Animatable, PartialEq)]
struct Look {
    scale: f32,
    tint: Color,
}

/// A round toolbar button holding an icon, tinted while hovered. A floating
/// button also swells while pressed, and both the swell and the tint ease in and
/// out rather than snapping.
#[derive(Clone, StatelessWidget)]
pub struct IconButton {
    icon: &'static StaticSvg,
    on_press: VoidCallback,
    size: f32,
    glass: bool,
}

impl IconButton {
    pub fn new(icon: &'static StaticSvg, on_press: impl Into<VoidCallback>) -> Self {
        Self {
            icon,
            on_press: on_press.into(),
            size: 36.0,
            glass: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Floats the button as a disc of liquid glass, for over a page.
    pub fn glass(mut self) -> Self {
        self.glass = true;
        self
    }
}

impl StatelessWidget for IconButton {
    fn build(&self, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let (data, size, floating) = (self.icon, self.size, self.glass);
        Button::new()
            .no_decor()
            .on_press(self.on_press.clone())
            .build(move |s| {
                ImplicitAnimatedBuilder::new(look(&s, theme, floating), DURATION, curves::FAST_OUT_SLOW_IN, move |look| {
                    Scalable::new().scale(look.scale).child(
                        Container::new()
                            .width(size)
                            .height(size)
                            .decoration(if floating {
                                vec![
                                    glass(theme).notype(),
                                    BorderRadius::all(size / 2.0).notype(),
                                    look.tint.notype(),
                                ]
                            } else {
                                vec![BorderRadius::all(size / 2.0).notype(), look.tint.notype()]
                            })
                            .child(Align::new().alignment(Alignment::MidCenter).child(icon(
                                data,
                                (size * 0.5) as u32,
                                theme.text,
                            ))),
                    )
                })
            })
    }
}

/// How the button looks while it is pressed, hovered, or at rest.
fn look(s: &ButtonVisualState, theme: NoteTheme, floating: bool) -> Look {
    if s.is_pressed {
        Look { scale: if floating { PRESSED_SCALE } else { 1.0 }, tint: theme.hover.with_alpha(0.2) }
    } else if s.is_hovered {
        Look { scale: 1.0, tint: theme.hover }
    } else {
        Look { scale: 1.0, tint: Color::Transparent }
    }
}
