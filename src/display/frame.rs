use std::fmt;
use termion::{color, cursor};

use super::{bounds::Bounds, format_str::FormattedString, Widget};
use crate::utils::usize::BoundedSub;

#[derive(Debug)]
pub struct FrameStyle {
    pub top_left: char,
    pub top: char,
    pub top_right: char,
    pub left: char,
    pub right: char,
    pub bottom_left: char,
    pub bottom: char,
    pub bottom_right: char,
    pub title_left: char,
    pub title_right: char,
}

pub static SINGLE: FrameStyle = FrameStyle {
    top_left: '┌',
    top: '─',
    top_right: '┐',
    left: '│',
    right: '│',
    bottom_left: '└',
    bottom: '─',
    bottom_right: '┘',
    title_left: '┤',
    title_right: '├',
};

pub static DOUBLE: FrameStyle = FrameStyle {
    top_left: '╔',
    top: '═',
    top_right: '╗',
    left: '║',
    right: '║',
    bottom_left: '╚',
    bottom: '═',
    bottom_right: '╝',
    title_left: '╡',
    title_right: '╞',
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct Title(pub FormattedString, pub Direction);

#[derive(Debug)]
pub struct FrameWidget<'a> {
    pub bounds: Bounds,
    pub top_title: Option<Title>,
    pub bottom_title: Option<Title>,
    pub frame_style: &'a FrameStyle,
}

impl<'a> Widget for FrameWidget<'a> {
    fn bounds(&self) -> Bounds {
        self.bounds
    }
}

impl<'a> fmt::Display for FrameWidget<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let blank_width = (self.bounds.width() as usize).bounded_sub(2);

        let top = if let Some(ref title) = self.top_title {
            format_with_title(
                title,
                blank_width,
                self.frame_style.top,
                self.frame_style.title_left,
                self.frame_style.title_right,
            )
        } else {
            self.frame_style.top.to_string().repeat(blank_width)
        };

        let bottom = if let Some(ref title) = self.bottom_title {
            format_with_title(
                title,
                blank_width,
                self.frame_style.bottom,
                self.frame_style.title_left,
                self.frame_style.title_right,
            )
        } else {
            self.frame_style.bottom.to_string().repeat(blank_width)
        };

        let goto: cursor::Goto = self.bounds.top_left.into();
        let step = format!(
            "{}{}",
            cursor::Down(1),
            cursor::Left(self.bounds.width() as u16)
        );
        let white = color::Fg(color::White);

        write!(
            fmt,
            "{goto}{white}{top_left}{top}{white}{top_right}",
            goto = goto,
            white = white,
            top_left = self.frame_style.top_left,
            top = top,
            top_right = self.frame_style.top_right,
        )?;

        for _ in 1..(self.bounds.height() - 1) {
            write!(
                fmt,
                "{step}{white}{left}{skip}{right}",
                step = step,
                white = white,
                left = self.frame_style.left,
                skip = cursor::Right(blank_width as u16),
                right = self.frame_style.right,
            )?;
        }

        write!(
            fmt,
            "{step}{white}{bottom_left}{bottom}{white}{bottom_right}",
            step = step,
            white = white,
            bottom_left = self.frame_style.bottom_left,
            bottom = bottom,
            bottom_right = self.frame_style.bottom_right,
        )?;

        Ok(())
    }
}

fn format_with_title(
    Title(text, direction): &Title,
    width: usize,
    filler: char,
    title_left: char,
    title_right: char,
) -> String {
    let formatted_title = format!("{} {} {}", title_left, text, title_right);
    let formatted_title_len = text.len() + 4;

    let available_len = width.bounded_sub(formatted_title_len);

    let (left_len, right_len) = match direction {
        Direction::Left => (1, available_len.bounded_sub(1)),
        Direction::Center => {
            let left_len = available_len / 2;
            let right_len = available_len - left_len;
            (left_len, right_len)
        }
        Direction::Right => (available_len.bounded_sub(1), 1),
    };

    format!(
        "{}{}{}",
        filler.to_string().repeat(left_len),
        formatted_title,
        filler.to_string().repeat(right_len),
    )
}
