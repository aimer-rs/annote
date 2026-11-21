use aimer::{Color, StaticSvg, Svg, SvgStyle};

/// A stroked Lucide icon in `color`.
pub fn icon(data: &'static StaticSvg, size: u32, color: Color) -> Svg {
    Svg::new(data)
        .width(size)
        .height(size)
        
        .style("path", SvgStyle::new().stroke(color).stroke_width(1.5))
}
