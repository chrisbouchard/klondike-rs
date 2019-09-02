use std::fmt;
use termion::{color, cursor};

use super::{
    bounds::Bounds,
    coords::Coords,
    format_str::FormattedString,
    frame::{self, Direction, FrameWidget, Title},
    Widget,
};

static MARGIN: Coords = Coords::from_xy(2, 1);
static BORDER: Coords = Coords::from_xy(1, 1);
static PADDING: Coords = Coords::from_xy(2, 1);

#[derive(Debug)]
pub struct HelpWidget {
    pub bounds: Bounds,
}

impl Widget for HelpWidget {
    fn bounds(&self) -> Bounds {
        self.bounds
    }
}

impl fmt::Display for HelpWidget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let frame_display = FrameWidget {
            bounds: self.bounds,
            top_title: Some(Title(
                FormattedString::of_content("H E L P"),
                Direction::Center,
            )),
            bottom_title: Some(Title(
                FormattedString::of_content("Press any key to continue . . ."),
                Direction::Right,
            )),
            frame_style: &frame::DOUBLE,
        };

        write!(fmt, "{}", frame_display)?;

        let cyan = color::Fg(color::Cyan);
        let white = color::Fg(color::White);
        let reset = color::Fg(color::Reset);

        let inner_top_left = self.bounds.top_left + BORDER + PADDING;
        let inner_bottom_right = self.bounds.bottom_right - BORDER - PADDING;
        let inner_bounds = Bounds::new(inner_top_left, inner_bottom_right);

        let inner_top_middle = inner_top_left + Coords::from_x(inner_bounds.width() / 2);

        write!(
            fmt,
            "{goto}{cyan}h{reset} / {cyan}j{reset} / {cyan}k{reset} / {cyan}l{reset} :  {white}Move",
            goto = cursor::Goto::from(inner_top_left),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}←{reset} / {cyan}↓{reset} / {cyan}↑{reset} / {cyan}→{reset} :  {white}Move",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(1)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}s{reset} :  {white}Move to Stock/Deck",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(3)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}t{reset} :  {white}Move to Talon/Waste",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(4)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}f{reset} :  {white}Move to Next Foundation",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(5)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}-{reset} :  {white}Move to Previous",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(6)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}F1{reset} ... {cyan}F4{reset} :  {white}Move to Foundation",
            goto = cursor::Goto::from(inner_top_middle),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}1{reset} ... {cyan}7{reset} :  {white}Move to Tableaux",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(1)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}SPACE{reset} / {cyan}RETURN{reset} :  {white}Pick Up/Activate",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(3)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}ESC{reset} :  {white}Return Held Cards",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(4)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}?{reset} :  {white}Display Help",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(6)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            fmt,
            "{goto}{cyan}q{reset} :  {white}Quit",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(7)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        Ok(())
    }
}
