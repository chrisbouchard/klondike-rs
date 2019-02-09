use self::horizontal::HorizontalStackPainter;
use self::vertical::VerticalStackPainter;

mod common;
mod horizontal;
mod vertical;

pub trait StackPainter: HorizontalStackPainter + VerticalStackPainter {}

impl<T> StackPainter for T where T: HorizontalStackPainter + VerticalStackPainter {}
