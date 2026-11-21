use crate::components::glass::glass;
use crate::components::icon::icon;
use crate::components::pad::pad;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::animation::{ImplicitAnimatedBuilder, curves};
use aimer::decoration::Decoration;
use aimer::input::TextEditingController;
use aimer::input::input::TextField;
use aimer::style::{
    BorderSlice, BorderStyle, BoxBorder, BoxDecoration, LayoutSpacing, TextStyle, Theme,
};
use aimer::{
    BoxAlignment, BuildContext, Container, Dimension, Expanded, Icons, ProviderHandle, Row,
    Scalable, Stack, State, StateView, StatefulWidget, Widget, ZeroSizedBox,
};
use std::time::Duration;

const WIDTH: f32 = 280.0;
const HEIGHT: f32 = 34.0;
/// How much larger the field is, at most, while it has focus.
const FOCUS_SCALE: f32 = 0.03;
/// How long the field takes to come into focus and to leave it.
const FOCUS_DURATION: Duration = Duration::from_millis(280);
/// The thickness of the focus ring around the field.
const RING_WIDTH: u32 = 2;

/// The search field at the right of the toolbar. It filters the note list.
///
/// Focus lifts the field: it swells a little, a ring of the accent colour draws
/// around it and the magnifier takes the accent too, all easing in together and
/// out again on blur.
#[derive(Clone, StatefulWidget)]
pub struct SearchBox;

struct SearchBoxState {
    controller: TextEditingController,
    /// Whether the field has the keyboard, which lifts it.
    focused: bool,
}

impl State<SearchBox> for SearchBoxState {
    fn create(_: SearchBox) -> Self {
        Self {
            controller: TextEditingController::new(),
            focused: false,
        }
    }

    fn build(this: StateView<Self>, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let store = ProviderHandle::<NoteStore>::of(ctx);
        let controller = this.controller.clone();
        let (focus_in, focus_out) = (this.updater(), this.updater());

        ImplicitAnimatedBuilder::new(
            if this.focused { 1.0 } else { 0.0 },
            FOCUS_DURATION,
            curves::FAST_OUT_SLOW_IN,
            move |focus| {
                let focus = *focus;
                let (focus_in, focus_out) = (focus_in.clone(), focus_out.clone());
                let (controller, store) = (controller.clone(), store.clone());

                // The ring is drawn over the glass's rim, always as thick, and only
                // its colour comes and goes, so nothing moves when it does.
                let ring = Container::new()
                    .width(Dimension::Percent(100.0))
                    .height(Dimension::Px(HEIGHT))
                    .box_decoration(
                        BoxDecoration::new()
                            .border(BoxBorder::all(
                                BorderSlice::new()
                                    .style(BorderStyle::Solid)
                                    .stroke(RING_WIDTH)
                                    .color(theme.accent.with_alpha(focus)),
                            ))
                            .border_radius(HEIGHT / 2.0),
                    )
                    .child(ZeroSizedBox);

                let pill = Container::new()
                    .width(Dimension::Percent(100.0))
                    .height(Dimension::Px(HEIGHT))
                    .decoration([glass(theme, HEIGHT).notype(), theme.text.with_alpha(0.03).notype()])
                    .padding(pad(2, 10, 0, 10))
                    .child(
                        Row::new()
                            .vertical_alignment(BoxAlignment::Center)
                            .gaps(LayoutSpacing::all(6))
                            .children([
                                icon(
                                    Icons::SEARCH,
                                    16,
                                    theme
                                        .muted
                                        .lerp(theme.accent, focus),
                                )
                                .boxed(),
                                Expanded::new()
                                    .child(
                                        TextField::new()
                                            .controller(controller)
                                            .hint("Search")
                                            .hint_style(
                                                TextStyle::new()
                                                    .font_size(14)
                                                    .color(theme.muted),
                                            )
                                            .text_style(
                                                TextStyle::new()
                                                    .font_size(14)
                                                    .color(theme.text),
                                            )
                                            .decoration(BoxDecoration::new())
                                            .padding(LayoutSpacing::vertical(7))
                                            .on_changed(move |text: String| {
                                                store.update(|s| s.set_search(text))
                                            })
                                            .on_focus(move |_: String| {
                                                focus_in.set_state(|s| s.focused = true)
                                            })
                                            .on_blur(move |_: String| {
                                                focus_out.set_state(|s| s.focused = false)
                                            }),
                                    )
                                    .boxed(),
                            ]),
                    );

                Container::new()
                    .width(WIDTH)
                    .height(HEIGHT)
                    .child(
                        Scalable::new()
                            .scale(1.0 + FOCUS_SCALE * focus)
                            .child(Stack::new().children([pill.boxed(), ring.boxed()])),
                    )
            },
        )
    }
}
