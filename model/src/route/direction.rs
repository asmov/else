use std::{fmt, str::FromStr};
use serde;
use strum;
use crate::{error::*, modeling::*, codebase::*, identity::*};

#[derive(PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize, Debug, strum::Display, strum::EnumString)]
pub enum HorizontalDirection {
    Vertical  = -1,
    North     = 0,
    NorthEast = 45,
    East      = 90,
    SouthEast = 135,
    South     = 180,
    SouthWest = 225,
    West      = 270,
    NorthWest = 315,
}

#[derive(PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize, Debug, strum::Display, strum::EnumString)]
pub enum VerticalDirection {
    Down  = -1,
    Level = 0,
    Up    = 1,
}

#[derive(Eq, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize, Debug)]
pub struct Direction {
    horizontal: HorizontalDirection,
    vertical: VerticalDirection
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_level() {
            write!(f, "{}", self.horizontal)
        } else {
            write!(f, "{} & {}", self.horizontal, self.vertical)
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('&').collect();
        let horizontal: HorizontalDirection = parts[0].trim().parse()
            .map_err(|_| Error::Parsing{ what: "HorizontalDirection",  src: s.to_string() } )?;
        let vertical = if parts.len() > 1 {
            parts[1].trim().parse()
                .map_err(|_| Error::Parsing{ what: "VerticalDirection",  src: s.to_string() } )?
        } else {
            VerticalDirection::Level
        };

        Ok(Self {
            horizontal,
            vertical
        })
    }
}

impl NonPrimitive for Direction {
    fn class_ident(&self) -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

impl Direction {
    const CLASSNAME: &'static str = "Direction";
    const CLASS_IDENT: ClassIdent = ClassIdent::new(CodebaseClassID::Direction as ClassID, "Direction");

    pub fn new(horizontal: HorizontalDirection, vertical: VerticalDirection) -> Self {
        Self {
            horizontal,
            vertical
        }
    }

    pub fn level(horizontal: HorizontalDirection) -> Self {
        Self {
            horizontal,
            vertical: VerticalDirection::Level
        }
    }

    pub fn up() -> Self {
        Self {
            horizontal: HorizontalDirection::Vertical,
            vertical: VerticalDirection::Up
        }
    }

    pub fn down() -> Self {
        Self {
            horizontal: HorizontalDirection::Vertical,
            vertical: VerticalDirection::Down
        }
    }

    pub fn h(&self) -> HorizontalDirection {
        self.horizontal
    }

    pub fn v(&self) -> VerticalDirection {
        self.vertical
    }

    pub fn horizontal(&self) -> HorizontalDirection {
        self.horizontal
    }

    pub fn vertical(&self) -> VerticalDirection {
        self.vertical
    }

    pub fn is_level(&self) -> bool {
        self.vertical == VerticalDirection::Level
    }

    pub const fn class_ident_const() -> &'static ClassIdent {
        &Self::CLASS_IDENT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_from_str() {
        let d: Direction = "North".parse().unwrap();
        assert_eq!(d.horizontal, HorizontalDirection::North);
        assert_eq!(d.vertical, VerticalDirection::Level);

        let d: Direction = "North & Up".parse().unwrap();
        assert_eq!(d.horizontal, HorizontalDirection::North);
        assert_eq!(d.vertical, VerticalDirection::Up);

        let d: Direction = "North & Down".parse().unwrap();
        assert_eq!(d.horizontal, HorizontalDirection::North);
        assert_eq!(d.vertical, VerticalDirection::Down);
    }
}

