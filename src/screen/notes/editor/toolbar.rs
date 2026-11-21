mod search_box;

use self::search_box::SearchBox;
use super::TOOLBAR_HEIGHT;
use crate::components::icon_button::IconButton;
use crate::components::pad::pad;
use crate::store::NoteStore;
use crate::theme::NoteTheme;
use aimer::animation::{ImplicitAnimatedBuilder, curves};
use aimer::decoration::{BackdropFilter, Decoration};
use aimer::input::mouse_region::MouseRegion;
use aimer::style::{BorderSlice, BorderStyle, BoxBorder, LayoutSpacing, Theme};
use aimer::{
    BoxAlignment, BuildContext, Color, Container, Dimension, Expanded, Icons, Opacity,
    ProviderContext, ProviderHandle, Row, SizedBox, Stack, State, StateView, StatefulWidget,
    Widget, ZeroSizedBox, list,
};
use std::time::Duration;

/// How opaque the blurred backdrop is once the note has scrolled under the bar.
const BACKDROP_OPACITY: f32 = 0.8;
/// How opaque it is while the pointer is over the bar.
const BACKDROP_HOVER_OPACITY: f32 = 0.9;
/// How long the backdrop takes to change its opacity.
const BACKDROP_DURATION: Duration = Duration::from_millis(330);
/// How much the page behind the bar is blurred, in logical pixels.
const BACKDROP_BLUR: f32 = 8.0;
/// Below this the backdrop is not worth a blur pass of its own.
const BACKDROP_FLOOR: f32 = 0.01;

/// The buttons above the note, and the search field.
///
/// Once the note has scrolled under the bar (`scrolled`), the bar gets a blurred
/// backdrop, so what passes beneath it does not fight with the buttons. The
/// backdrop firms up while the pointer is over the bar.
#[derive(Clone, StatefulWidget)]
pub struct Toolbar {
    scrolled: bool,
}

impl Toolbar {
    pub fn new(scrolled: bool) -> Self {
        Self { scrolled }
    }
}

struct ToolbarState {
    scrolled: bool,
    /// Whether the pointer is over the bar, buttons and all.
    hovered: bool,
}

impl ToolbarState {
    /// How opaque the backdrop is: none until the note has scrolled, then firmer
    /// under the pointer.
    fn backdrop_opacity(&self) -> f32 {
        match (self.scrolled, self.hovered) {
            (false, _) => 0.0,
            (true, false) => BACKDROP_OPACITY,
            (true, true) => BACKDROP_HOVER_OPACITY,
        }
    }
}

impl State<Toolbar> for ToolbarState {
    fn create(widget: Toolbar) -> Self {
        Self {
            scrolled: widget.scrolled,
            hovered: false,
        }
    }

    fn adopt_config_from(&mut self, new: Self) {
        self.scrolled = new.scrolled;
    }

    fn build(this: StateView<Self>, ctx: &BuildContext) -> impl Widget {
        let theme = *NoteTheme::of(ctx);
        let (enter, exit) = (this.updater(), this.updater());
        let opacity = this.backdrop_opacity();
        let store = ProviderHandle::<NoteStore>::of(ctx);
        let selected = ctx.select::<NoteStore, Option<u64>>(|s| s.selected_note);
        let pinned = ctx.select::<NoteStore, bool>(|s| {
            s.selected_note
                .and_then(|id| s.note(id))
                .is_some_and(|n| n.pinned)
        });
        let (new_store, pin_store, delete_store, picture_store) =
            (store.clone(), store.clone(), store.clone(), store.clone());

        // A fixed height: floating over the page, the bar would otherwise be as tall as
        // the page, and its row would centre the buttons in it.
        let bar = Container::new()
            .height(TOOLBAR_HEIGHT)
            .padding(pad(10, 12, 10, 12))
            .child(
                Row::new()
                    .vertical_alignment(BoxAlignment::Center)
                    .gaps(LayoutSpacing::all(6))
                    .children(list![
                        IconButton::new(Icons::SQUARE_PEN, move || {
                            new_store.update(|s| s.new_note())
                        })
                        .glass(),
                        IconButton::new(
                            if pinned { Icons::PIN_OFF } else { Icons::PIN },
                            move || {
                                if let Some(id) = selected {
                                    pin_store.update(|s| s.toggle_pin(id));
                                }
                            },
                        )
                        .glass(),
                        IconButton::new(Icons::TRASH, move || {
                            if let Some(id) = selected {
                                delete_store.update(|s| s.delete_note(id));
                            }
                        })
                        .glass(),
                        IconButton::new(Icons::IMAGE_PLUS, move || {
                            if let Some(id) = selected {
                                crate::images::pick_and_insert(picture_store.clone(), id);
                            }
                        })
                        .glass(),
                        Expanded::new().child(SizedBox::new()),
                        SearchBox,
                    ]),
            );

        // The region wraps the buttons too, so the pointer stays "over the bar" while
        // it is on one of them.
        MouseRegion::new()
            .on_hover_enter(move || enter.set_state(|s| s.hovered = true))
            .on_hover_exit(move || exit.set_state(|s| s.hovered = false))
            .child(Stack::new().children([backdrop(theme, opacity), bar.boxed()]))
    }
}

/// The blurred band behind the bar, `opacity` opaque, easing from the last one.
fn backdrop(theme: NoteTheme, opacity: f32) -> aimer::AnyWidget {
    ImplicitAnimatedBuilder::new(
        opacity,
        BACKDROP_DURATION,
        curves::LINEAR,
        move |opacity| {
            let opacity = *opacity;
            // The blur is only worth a pass while it can be seen.
            let blur = if opacity > BACKDROP_FLOOR {
                [

                    theme
                        .editor
                        .with_alpha(opacity)
                        .notype(),
                        
                    BackdropFilter::new()
                        .blur_radius(BACKDROP_BLUR)
                        .brightness(0.9)
                        .notype(),
                ]
                .notype()
            } else {
                Color::Transparent.notype()
            };
            Container::new()
                .width(Dimension::Percent(100.0))
                .height(Dimension::Px(TOOLBAR_HEIGHT as f32))
                .decoration(blur)
                .child(ZeroSizedBox)
        },
    )
    .boxed()
}
