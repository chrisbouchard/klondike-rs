use std::{fmt, io};
use termion::{color, cursor};

use super::{bounds::Bounds, coords::Coords, error::Result};
use crate::utils::usize::BoundedSub;

#[derive(Debug)]
pub struct FrameStyle {
    top_left: char,
    top: char,
    top_right: char,
    left: char,
    right: char,
    bottom_left: char,
    bottom: char,
    bottom_right: char,
    title_left: char,
    title_right: char,
}

pub static SINGLE_CURVED: FrameStyle = FrameStyle {
    top_left: '╭',
    top: '─',
    top_right: '╮',
    left: '│',
    right: '│',
    bottom_left: '╰',
    bottom: '─',
    bottom_right: '╯',
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
pub struct Title<D>(pub D, pub Direction)
where
    D: fmt::Display;

pub trait FramePainter {
    fn draw_frame(
        &mut self,
        bounds: Bounds,
        top_title: Option<Title<impl fmt::Display>>,
        bottom_title: Option<Title<impl fmt::Display>>,
        frame_style: &FrameStyle,
    ) -> Result<()>;
}

impl<W> FramePainter for W
where
    W: io::Write,
{
    fn draw_frame(
        &mut self,
        bounds: Bounds,
        top_title: Option<Title<impl fmt::Display>>,
        bottom_title: Option<Title<impl fmt::Display>>,
        frame_style: &FrameStyle,
    ) -> Result<()> {
        let blank_width = (bounds.width() as usize).bounded_sub(2);

        let top = if let Some(title) = top_title {
            draw_frame_with_title(
                title,
                blank_width,
                frame_style.top,
                frame_style.title_left,
                frame_style.title_right,
            )?
        } else {
            frame_style.top.to_string().repeat(blank_width)
        };

        let bottom = if let Some(title) = bottom_title {
            draw_frame_with_title(
                title,
                blank_width,
                frame_style.bottom,
                frame_style.title_left,
                frame_style.title_right,
            )?
        } else {
            frame_style.bottom.to_string().repeat(blank_width)
        };

        let white = color::Fg(color::White);

        write!(
            self,
            "{goto}{white}{top_left}{top}{top_right}",
            goto = cursor::Goto::from(bounds.top_left),
            white = white,
            top_left = frame_style.top_left,
            top = top,
            top_right = frame_style.top_right,
        )?;

        for i in 1..(bounds.height() - 1) {
            write!(
                self,
                "{goto}{white}{left}{skip}{right}",
                goto = cursor::Goto::from(bounds.top_left + Coords::from_y(i)),
                white = white,
                left = frame_style.left,
                skip = cursor::Right(blank_width as u16),
                right = frame_style.right,
            )?;
        }

        write!(
            self,
            "{goto}{white}{bottom_left}{bottom}{bottom_right}",
            goto = cursor::Goto::from(bounds.top_left.to_x() + bounds.bottom_right.to_y()),
            white = white,
            bottom_left = frame_style.bottom_left,
            bottom = bottom,
            bottom_right = frame_style.bottom_right,
        )?;

        Ok(())
    }
}

fn draw_frame_with_title(
    Title(text, direction): Title<impl fmt::Display>,
    width: usize,
    filler: char,
    title_left: char,
    title_right: char,
) -> Result<String> {
    let formatted_title = format!("{} {} {}", title_left, text, title_right,);

    let available_len = width.bounded_sub(formatted_title.chars().count());

    let (left_len, right_len) = match direction {
        Direction::Left => (1, available_len.bounded_sub(1)),
        Direction::Center => {
            let left_len = available_len / 2;
            let right_len = available_len - left_len;
            (left_len, right_len)
        }
        Direction::Right => (available_len.bounded_sub(1), 1),
    };

    Ok(format!(
        "{}{}{}",
        filler.to_string().repeat(left_len),
        formatted_title,
        filler.to_string().repeat(right_len),
    ))
}
