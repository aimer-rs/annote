use crate::theme::NoteTheme;
use aimer::canvas::{Canvas, Rect};
use aimer::decoration::*;
use aimer::{Color, ResolvedSize, Vec2d};

/// The liquid glass the floating toolbar is made of: the page behind it blurred,
/// a lens that bends what is under the rim, and a light edge, all inside a pill.
pub fn glass(theme: NoteTheme, size: f32) -> impl Decoration {
    type Style = MagnificationStyle;
    GlassDecoration::new(theme.list.with_alpha(0.5))
        .light_edge(LightEdge::new().width(1.0).strength(0.6))
        .lens(
            Magnification::new()
                .zoom(1.15)
                .thickness(24.0)
                .bulge(0.05)
                .bevel_width(12.0)
                .bulge_radius(size / 2.0)
                .style(Style::REFRACTION | Style::BULGE | Style::UNIFORM),
        )
        .backdrop(
            BackdropFilter::new()
                .blur_radius(6.0)
                .saturation(0.65)
                .brightness(0.9)
                .contrast(0.8),
        )
}

/// A pill of frosted glass: the page behind it blurred, a translucent tint over
/// that, and a light edge along its rim.
///
/// The backdrop filter only knows rectangles, so the blur is painted inside a
/// clip with the pill's corners. The corners are always half the shorter side, so
/// the same decoration shapes a wide search field and a round button.
pub struct GlassDecoration {
    tint: Color,
    filter: BackdropFilter,
    edge: LightEdge,
    lens: Magnification,
}

impl GlassDecoration {
    /// Glass tinted with `tint`, whose alpha says how much of the blurred page
    /// shows through.
    pub fn new(tint: Color) -> Self {
        Self {
            tint,
            filter: BackdropFilter::new().blur_radius(18.0).saturation(1.5),
            edge: LightEdge::new().width(6.0).strength(0.7),
            lens: Magnification::new(),
        }
    }

    pub fn lens(mut self, lens: Magnification) -> Self {
        self.lens = lens;
        self
    }

    pub fn backdrop(mut self, backdrop: BackdropFilter) -> Self {
        self.filter = backdrop;
        self
    }

    pub fn light_edge(mut self, edge: LightEdge) -> Self {
        self.edge = edge;
        self
    }

    /// The tint layer, shaped as a pill of `radius`.
    fn body(&self, radius: f32) -> BoxDecoration {
        BoxDecoration::new().background_color(self.tint).border_radius(radius)
    }
}

impl Decoration for GlassDecoration {
    fn paint(&self, canvas: &Canvas, origin: Vec2d, size: ResolvedSize, scale: f32) {
        if size.width <= 0.0 || size.height <= 0.0 {
            return;
        }
        let radius = size.width.min(size.height) * 0.5;

        canvas.push_clip(Rect::new(origin.x, origin.y, size.width, size.height), [radius; 4]);
        self.body(radius).paint_layer(canvas, origin, size, scale);
        self.filter.paint(canvas, origin, size, scale);
        self.lens.paint(canvas, origin, size, scale);
        canvas.pop_clip();

        self.edge.paint_layer(canvas, origin, size, scale);
    }

    fn paint_outsets(&self, size: ResolvedSize, scale: f32) -> [f32; 4] {
        self.body(size.width.min(size.height) * 0.5).paint_outsets(size, scale)
    }

    /// The tint, so the glass stops scroll events as an opaque bar would.
    fn background_color(&self) -> Option<Color> {
        Some(self.tint)
    }
}
