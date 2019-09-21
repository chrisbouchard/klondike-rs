use termion::cursor;

pub use euclid::{
    default::{Box2D, Point2D, Rect, SideOffsets2D, Size2D, Vector2D},
    point2, rect, size2, vec2, NonEmpty,
};

pub fn goto(point: Point2D<u16>) -> cursor::Goto {
    let (x, y) = point.to_tuple();
    cursor::Goto(x + 1, y + 1)
}
