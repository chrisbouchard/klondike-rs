use itertools::Itertools;
use std::{convert::TryFrom, fmt};
use termion::color;

use crate::utils::{format_str::FormattedString, str::CharacterLength};

use super::{
    frame::{self, FrameWidget, Title},
    geometry, Widget,
};

lazy_static! {
    static ref MARGIN: geometry::SideOffsets2D<u16> = geometry::SideOffsets2D::new(1, 2, 1, 2);
    static ref BORDER: geometry::SideOffsets2D<u16> = geometry::SideOffsets2D::new_all_same(1);
    static ref PADDING: geometry::SideOffsets2D<u16> = geometry::SideOffsets2D::new(1, 2, 1, 2);
}

#[derive(Debug)]
pub struct HelpWidget {
    pub bounds: geometry::Rect<u16>,
}

impl Widget for HelpWidget {
    fn bounds(&self) -> geometry::Rect<u16> {
        self.bounds
    }
}

impl fmt::Display for HelpWidget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let frame_bounds = self.bounds.inner_rect(*MARGIN);
        let inner_bounds = frame_bounds.inner_rect(*BORDER + *PADDING);

        let frame_display = FrameWidget {
            bounds: frame_bounds,
            top_title: Some(Title::center("H E L P")),
            bottom_title: Some(Title::right("Press any key to continue . . .")),
            frame_style: &frame::DOUBLE,
        };

        write!(fmt, "{}", frame_display)?;

        let inner_top_left = inner_bounds.origin;
        let inner_top_middle = inner_top_left + geometry::vec2(inner_bounds.size.width / 2 + 1, 0);

        for item in left_column_items(inner_top_left) {
            write!(fmt, "{}", item)?;
        }

        for item in right_column_items(inner_top_middle) {
            write!(fmt, "{}", item)?;
        }

        Ok(())
    }
}

fn left_column_items(origin: geometry::Point2D<u16>) -> Vec<HelpItemWidget> {
    let mut coord_iter = (0..).map(|index| origin + geometry::vec2(0, index));

    vec![
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::List(vec!["h", "j", "k", "l"]),
            description: "Move",
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::List(vec!["←", "↓", "↑", "→"]),
            description: "Move",
        },
        HelpItemWidget::Skip {
            origin: coord_iter.next().unwrap(),
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("s"),
            description: "Go to Stock/Deck",
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("t"),
            description: "Go to Talon/Waste",
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("f"),
            description: "Go to next Foundation",
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("-"),
            description: "Go back to previous area",
        },
    ]
}

fn right_column_items(origin: geometry::Point2D<u16>) -> Vec<HelpItemWidget> {
    let mut coord_iter = (0..).map(|index| origin + geometry::vec2(0, index));

    vec![
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Range("F1", "F4"),
            description: "Go to Foundation",
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Range("1", "7"),
            description: "Go to Tableaux",
        },
        HelpItemWidget::Skip {
            origin: coord_iter.next().unwrap(),
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::List(vec!["SPACE", "RETURN"]),
            description: "Pick Up/Activate",
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("ESC"),
            description: "Return Held Cards",
        },
        HelpItemWidget::Skip {
            origin: coord_iter.next().unwrap(),
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("?"),
            description: "Help",
        },
        HelpItemWidget::Mapping {
            origin: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("q"),
            description: "Quit",
        },
    ]
}

#[derive(Debug)]
enum HelpItemKeys {
    Single(&'static str),
    List(Vec<&'static str>),
    Range(&'static str, &'static str),
}

impl HelpItemKeys {
    fn len(&self) -> usize {
        match self {
            Self::Single(key) => key.char_len(),
            Self::List(keys) => keys.iter().map(|key| key.char_len()).intersperse(3).sum(),
            Self::Range(start_key, end_key) => start_key.char_len() + end_key.char_len() + 5,
        }
    }
}

impl fmt::Display for HelpItemKeys {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Single(key) => {
                write!(fmt, "{key_style}{key}", key_style = key_style(), key = key)?;
            }
            Self::List(keys) => {
                let formatted_keys_iter = keys
                    .iter()
                    .map(|key| FormattedString::new_with_formatting(key_style()).push_content(key))
                    .intersperse(
                        FormattedString::new_with_formatting(reset_style()).push_content(" / "),
                    );

                for formatted_key in formatted_keys_iter {
                    write!(fmt, "{}", formatted_key)?;
                }
            }
            Self::Range(start_key, end_key) => {
                write!(
                    fmt,
                    "{key_style}{start_key}{reset} ... {key_style}{end_key}",
                    reset = reset_style(),
                    key_style = key_style(),
                    start_key = start_key,
                    end_key = end_key,
                )?;
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
enum HelpItemWidget {
    Mapping {
        origin: geometry::Point2D<u16>,
        keys: HelpItemKeys,
        description: &'static str,
    },
    Skip {
        origin: geometry::Point2D<u16>,
    },
}

impl Widget for HelpItemWidget {
    fn bounds(&self) -> geometry::Rect<u16> {
        match self {
            Self::Mapping {
                origin,
                keys,
                description,
            } => {
                let length = u16::try_from(keys.len() + description.char_len() + 4).unwrap();
                geometry::Rect::new(*origin, geometry::size2(length, 0))
            }
            Self::Skip { origin } => geometry::Rect::new(*origin, geometry::Size2D::zero()),
        }
    }
}

impl fmt::Display for HelpItemWidget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Self::Mapping {
            origin,
            keys,
            description,
        } = self
        {
            let goto = geometry::goto(*origin);

            write!(
                fmt,
                "{goto}{keys}{reset} :  {desc_style}{desc}",
                goto = goto,
                reset = reset_style(),
                desc_style = description_style(),
                keys = keys,
                desc = description,
            )?;
        }

        Ok(())
    }
}

fn key_style() -> impl fmt::Display {
    color::Fg(color::Cyan)
}

fn reset_style() -> impl fmt::Display {
    color::Fg(color::Reset)
}

fn description_style() -> impl fmt::Display {
    color::Fg(color::White)
}
