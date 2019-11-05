use std::fmt;

use super::{
    frame::{self, FrameWidget, Title},
    geometry, Widget,
};

lazy_static! {
    static ref CONTENT_SIZE: geometry::Size2D<u16> = geometry::size2(20, 3);
    static ref BORDER: geometry::SideOffsets2D<u16> = geometry::SideOffsets2D::new_all_same(1);
    static ref PADDING: geometry::SideOffsets2D<u16> = geometry::SideOffsets2D::new(1, 2, 1, 2);
}

#[derive(Debug)]
pub struct WinWidget {
    pub bounds: geometry::Rect<u16>,
}

impl Widget for WinWidget {
    fn bounds(&self) -> geometry::Rect<u16> {
        self.bounds
    }
}

impl fmt::Display for WinWidget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let left_offset = self.bounds.size.width.saturating_sub(CONTENT_SIZE.width) / 2;
        let top_offset = self.bounds.size.height.saturating_sub(CONTENT_SIZE.height) / 2;
        let offset: geometry::Vector2D<u16> = geometry::vec2(left_offset, top_offset);
        let inner_origin = self.bounds.origin + offset;

        let inner_bounds = geometry::Rect::new(inner_origin, *CONTENT_SIZE);
        let frame_bounds = inner_bounds.outer_rect(*BORDER + *PADDING);

        let frame_display = FrameWidget {
            bounds: frame_bounds,
            top_title: Some(Title::center("Y O U   W I N !")),
            bottom_title: None,
            frame_style: &frame::DOUBLE,
        };

        write!(fmt, "{}", frame_display)?;

        let goto_line1 = geometry::goto(inner_origin);
        let goto_line2 = geometry::goto(inner_origin + geometry::vec2(0, 2));

        write!(fmt, "{}Congratulations!", goto_line1)?;
        write!(fmt, "{}New Game? (Y/N)", goto_line2)?;

        Ok(())
    }
}
