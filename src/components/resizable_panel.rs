use aimer::{Color, Dimension, Direction, Expanded, Resizable, Row, SizedBox, Widget};

/// A column the pointer can resize by its right edge, with a 1px divider along
/// that edge. `content` fills the room that is left of the divider.
pub fn resizable_panel(
    content: impl Widget + 'static,
    start: f32,
    min: f32,
    max: f32,
    divider: Color,
) -> impl Widget {
    Resizable::new()
        .height(f32::MAX)
        .width(start)
        .min_width(min)
        .max_width(max)
        .handle_thickness(14.0)
        .handle_outset(7.0)
        .direction(Direction::RIGHT)
        .box_child(Row::new().children([
            Expanded::new().child(content).boxed(),
            SizedBox::new().width(1).height(Dimension::Percent(100.0)).color(divider).boxed(),
        ]))
}
