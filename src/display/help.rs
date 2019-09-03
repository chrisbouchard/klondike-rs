use itertools::Itertools;
use std::{convert::TryFrom, fmt};
use termion::{color, cursor};

use crate::utils::{
    bounds::Bounds,
    coords::{self, Coords},
    str::CharacterLength,
};

use super::{
    format_str::FormattedString,
    frame::{self, FrameWidget, Title},
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
        let frame_bounds = self.bounds.inset_by(MARGIN);
        let inner_bounds = frame_bounds.inset_by(BORDER + PADDING);

        let frame_display = FrameWidget {
            bounds: frame_bounds,
            top_title: Some(Title::center("H E L P")),
            bottom_title: Some(Title::right("Press any key to continue . . .")),
            frame_style: &frame::DOUBLE,
        };

        write!(fmt, "{}", frame_display)?;

        let inner_top_left = inner_bounds.top_left;
        let inner_top_middle = inner_top_left + Coords::from_x(inner_bounds.width() / 2 + 1);

        for item in left_column_items(inner_top_left) {
            write!(fmt, "{}", item)?;
        }

        for item in right_column_items(inner_top_middle) {
            write!(fmt, "{}", item)?;
        }

        Ok(())
    }
}

fn left_column_items(coords: Coords) -> Vec<HelpItemWidget> {
    let mut coord_iter = (0..).map(|index| coords + Coords::from_y(index));

    vec![
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::List(vec!["h", "j", "k", "l"]),
            description: "Move",
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::List(vec!["←", "↓", "↑", "→"]),
            description: "Move",
        },
        HelpItemWidget::Skip {
            coords: coord_iter.next().unwrap(),
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("s"),
            description: "Go to Stock/Deck",
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("t"),
            description: "Go to Talon/Waste",
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("f"),
            description: "Go to next Foundation",
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("-"),
            description: "Go back to previous area",
        },
    ]
}

fn right_column_items(coords: Coords) -> Vec<HelpItemWidget> {
    let mut coord_iter = (0..).map(|index| coords + Coords::from_y(index));

    vec![
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Range("F1", "F4"),
            description: "Go to Foundation",
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Range("1", "7"),
            description: "Go to Tableaux",
        },
        HelpItemWidget::Skip {
            coords: coord_iter.next().unwrap(),
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::List(vec!["SPACE", "RETURN"]),
            description: "Pick Up/Activate",
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("ESC"),
            description: "Return Held Cards",
        },
        HelpItemWidget::Skip {
            coords: coord_iter.next().unwrap(),
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
            keys: HelpItemKeys::Single("?"),
            description: "Help",
        },
        HelpItemWidget::Mapping {
            coords: coord_iter.next().unwrap(),
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
        coords: Coords,
        keys: HelpItemKeys,
        description: &'static str,
    },
    Skip {
        coords: Coords,
    },
}

impl Widget for HelpItemWidget {
    fn bounds(&self) -> Bounds {
        match self {
            Self::Mapping {
                coords,
                keys,
                description,
            } => {
                let length = i32::try_from(keys.len() + description.char_len() + 4).unwrap();
                Bounds::with_size(*coords, Coords::from_x(length))
            }
            Self::Skip { coords } => Bounds::new(*coords, *coords),
        }
    }
}

impl fmt::Display for HelpItemWidget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Self::Mapping {
            coords,
            keys,
            description,
        } = self
        {
            let goto: cursor::Goto = (*coords).into();

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
