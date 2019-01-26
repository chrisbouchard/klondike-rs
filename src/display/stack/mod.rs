pub use self::horizontal::HorizontalStackPainter;
pub use self::vertical::VerticalStackPainter;

mod common;
pub mod horizontal;
pub mod vertical;

pub trait StackPainter: HorizontalStackPainter + VerticalStackPainter {}

impl<T> StackPainter for T where T: HorizontalStackPainter + VerticalStackPainter {}
