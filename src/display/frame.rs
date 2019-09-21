use std::{convert::TryInto, fmt};
use termion::{color, cursor};

use crate::utils::format_str::FormattedString;

use super::{geometry, Widget};

#[derive(Debug)]
pub struct FrameStyle {
    pub top_left: &'static str,
    pub top: &'static str,
    pub top_right: &'static str,
    pub left: &'static str,
    pub right: &'static str,
    pub bottom_left: &'static str,
    pub bottom: &'static str,
    pub bottom_right: &'static str,
    pub title_left: &'static str,
    pub title_right: &'static str,
}

pub static SINGLE: FrameStyle = FrameStyle {
    top_left: "┌─",
    top: "─",
    top_right: "─┐",
    left: "│",
    right: "│",
    bottom_left: "└─",
    bottom: "─",
    bottom_right: "─┘",
    title_left: "┤ ",
    title_right: " ├",
};

pub static DOUBLE: FrameStyle = FrameStyle {
    top_left: "╔═",
    top: "═",
    top_right: "═╗",
    left: "║",
    right: "║",
    bottom_left: "╚═",
    bottom: "═",
    bottom_right: "═╝",
    title_left: "╡ ",
    title_right: " ╞",
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct Title(pub FormattedString, pub Direction);

impl Title {
    pub fn left<F>(text: F) -> Title
    where
        F: Into<FormattedString>,
    {
        Title(text.into(), Direction::Left)
    }

    pub fn center<F>(text: F) -> Title
    where
        F: Into<FormattedString>,
    {
        Title(text.into(), Direction::Center)
    }

    pub fn right<F>(text: F) -> Title
    where
        F: Into<FormattedString>,
    {
        Title(text.into(), Direction::Right)
    }
}

#[derive(Debug)]
pub struct FrameWidget<'a> {
    pub bounds: geometry::Rect<u16>,
    pub top_title: Option<Title>,
    pub bottom_title: Option<Title>,
    pub frame_style: &'a FrameStyle,
}

impl<'a> Widget for FrameWidget<'a> {
    fn bounds(&self) -> geometry::Rect<u16> {
        self.bounds
    }
}

impl<'a> fmt::Display for FrameWidget<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let width = self.bounds.size.width;

        let top_blank_width: u16 = usize::from(width)
            .saturating_sub(self.frame_style.top_left.chars().count())
            .saturating_sub(self.frame_style.top_right.chars().count())
            .try_into()
            .unwrap();

        let middle_blank_width: u16 = usize::from(width)
            .saturating_sub(self.frame_style.left.chars().count())
            .saturating_sub(self.frame_style.right.chars().count())
            .try_into()
            .unwrap();

        let bottom_blank_width: u16 = usize::from(width)
            .saturating_sub(self.frame_style.bottom_left.chars().count())
            .saturating_sub(self.frame_style.bottom_right.chars().count())
            .try_into()
            .unwrap();

        let top = if let Some(ref title) = self.top_title {
            format_with_title(
                title,
                top_blank_width,
                self.frame_style.top,
                self.frame_style.title_left,
                self.frame_style.title_right,
            )
        } else {
            self.frame_style
                .top
                .to_string()
                .repeat(top_blank_width.into())
        };

        let bottom = if let Some(ref title) = self.bottom_title {
            format_with_title(
                title,
                bottom_blank_width,
                self.frame_style.bottom,
                self.frame_style.title_left,
                self.frame_style.title_right,
            )
        } else {
            self.frame_style
                .bottom
                .to_string()
                .repeat(bottom_blank_width.into())
        };

        let goto = geometry::goto(self.bounds.origin);
        let step = format!("{}{}", cursor::Down(1), cursor::Left(width));
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

        for _ in 1..(self.bounds.size.height - 1) {
            write!(
                fmt,
                "{step}{white}{left}{skip}{right}",
                step = step,
                white = white,
                left = self.frame_style.left,
                skip = cursor::Right(middle_blank_width),
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

// TODO: What to do if filler doesn't divide evenly?
fn format_with_title(
    Title(text, direction): &Title,
    width: u16,
    filler: &str,
    title_left: &str,
    title_right: &str,
) -> String {
    let white = color::Fg(color::White);
    let formatted_title = FormattedString::new_with_content(title_left)
        .push_formatted_content(text)
        .push_formatting(white)
        .push_content(title_right);

    let available_len: u16 = usize::from(width)
        .saturating_sub(formatted_title.len())
        .try_into()
        .unwrap();

    let (left_len, right_len) = match direction {
        Direction::Left => (0, available_len),
        Direction::Center => {
            let left_len = available_len / 2;
            let right_len = available_len - left_len;
            (left_len, right_len)
        }
        Direction::Right => (available_len, 0),
    };

    format!(
        "{}{}{}",
        filler.to_string().repeat(left_len.into()),
        formatted_title,
        filler.to_string().repeat(right_len.into()),
    )
}
