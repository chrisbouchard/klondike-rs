use std::fmt;
use termion::{color, cursor};

use crate::utils::{bounds::Bounds, usize::BoundedSub};

use super::{format_str::FormattedString, Widget};

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
        let width = self.bounds.width() as usize;

        let top_blank_width = width
            .bounded_sub(self.frame_style.top_left.chars().count())
            .bounded_sub(self.frame_style.top_right.chars().count());

        let middle_blank_width = width
            .bounded_sub(self.frame_style.left.chars().count())
            .bounded_sub(self.frame_style.right.chars().count());

        let bottom_blank_width = width
            .bounded_sub(self.frame_style.bottom_left.chars().count())
            .bounded_sub(self.frame_style.bottom_right.chars().count());

        let top = if let Some(ref title) = self.top_title {
            format_with_title(
                title,
                top_blank_width,
                self.frame_style.top,
                self.frame_style.title_left,
                self.frame_style.title_right,
            )
        } else {
            self.frame_style.top.to_string().repeat(top_blank_width)
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
                .repeat(bottom_blank_width)
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
                skip = cursor::Right(middle_blank_width as u16),
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
    width: usize,
    filler: &str,
    title_left: &str,
    title_right: &str,
) -> String {
    let white = color::Fg(color::White);
    let formatted_title = FormattedString::of_content(title_left)
        .push_formatted_content(text)
        .push_formatting(white)
        .push_content(title_right);

    let available_len = width.bounded_sub(formatted_title.len());

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
        filler.to_string().repeat(left_len),
        formatted_title,
        filler.to_string().repeat(right_len),
    )
}
