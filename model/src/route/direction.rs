use std::{fmt, str::FromStr};
use serde;
use strum;
use crate::{error::*, modeling::*, codebase::*, identity::*};

#[derive(PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize, Debug, strum::Display, strum::EnumString)]
pub enum HorizontalDirection {
    #[strum(serialize = "Vertical", ascii_case_insensitive)]
    Vertical  = -1,
    #[strum(serialize = "North", ascii_case_insensitive)]
    North     = 0,
    #[strum(serialize = "Northeast", ascii_case_insensitive)]
    Northeast = 45,
    #[strum(serialize = "East", ascii_case_insensitive)]
    East      = 90,
    #[strum(serialize = "Southeast", ascii_case_insensitive)]
    Southeast = 135,
    #[strum(serialize = "South", ascii_case_insensitive)]
    South     = 180,
    #[strum(serialize = "Soutwest", ascii_case_insensitive)]
    Southwest = 225,
    #[strum(serialize = "West", ascii_case_insensitive)]
    West      = 270,
    #[strum(serialize = "Northwest", ascii_case_insensitive)]
    Northwest = 315,
}

impl HorizontalDirection {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Vertical  => "Vertical",
            Self::North     => "North",
            Self::Northeast => "Northeast",
            Self::East      => "East",
            Self::Southeast => "Southeast",
            Self::South     => "South",
            Self::Southwest => "Southwest",
            Self::West      => "West",
            Self::Northwest => "Northwest",
        }
    }

    pub fn name_lowercase(&self) -> &'static str {
        match self {
            Self::Vertical  => "vertical",
            Self::North     => "north",
            Self::Northeast => "northeast",
            Self::East      => "east",
            Self::Southeast => "southeast",
            Self::South     => "south",
            Self::Southwest => "southwest",
            Self::West      => "west",
            Self::Northwest => "northwest",
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize, Debug, strum::Display, strum::EnumString)]
pub enum VerticalDirection {
    #[strum(serialize = "Down", ascii_case_insensitive)]
    Down  = -1,
    #[strum(serialize = "Level", ascii_case_insensitive)]
    Level = 0,
    #[strum(serialize = "Up", ascii_case_insensitive)]
    Up    = 1,
}

impl VerticalDirection {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Down  => "Down",
            Self::Level => "Level",
            Self::Up    => "Up",
        }
    }
}



#[derive(Eq, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize, Debug)]
pub struct Direction {
    horizontal: HorizontalDirection,
    vertical: VerticalDirection
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
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

    pub fn name(&self) -> &'static str {
        match self.vertical {
            VerticalDirection::Level => self.horizontal.name(),
            VerticalDirection::Up =>  {
                match self.horizontal {
                    HorizontalDirection::Vertical => "Up",
                    HorizontalDirection::North => "Up North",
                    HorizontalDirection::Northeast => "Up Northeast",
                    HorizontalDirection::East => "Up East",
                    HorizontalDirection::Southeast => "Up Southeast",
                    HorizontalDirection::South => "Up South",
                    HorizontalDirection::Southwest => "Up Southwest",
                    HorizontalDirection::West => "Up West",
                    HorizontalDirection::Northwest => "Up Northwest",
                }
            },
            VerticalDirection::Down =>  {
                match self.horizontal {
                    HorizontalDirection::Vertical => "Down",
                    HorizontalDirection::North => "Down North",
                    HorizontalDirection::Northeast => "Down Northeast",
                    HorizontalDirection::East => "Down East",
                    HorizontalDirection::Southeast => "Down Southeast",
                    HorizontalDirection::South => "Down South",
                    HorizontalDirection::Southwest => "Down Southwest",
                    HorizontalDirection::West => "Down West",
                    HorizontalDirection::Northwest => "Down Northwest",
                }
            }
        }
    }

    pub fn name_lowercase(&self) -> &'static str {
        match self.vertical {
            VerticalDirection::Level => self.horizontal.name_lowercase(),
            VerticalDirection::Up =>  {
                match self.horizontal {
                    HorizontalDirection::Vertical => "up",
                    HorizontalDirection::North => "up north",
                    HorizontalDirection::Northeast => "up northeast",
                    HorizontalDirection::East => "up east",
                    HorizontalDirection::Southeast => "up southeast",
                    HorizontalDirection::South => "up south",
                    HorizontalDirection::Southwest => "up southwest",
                    HorizontalDirection::West => "up west",
                    HorizontalDirection::Northwest => "up northwest",
                }
            },
            VerticalDirection::Down =>  {
                match self.horizontal {
                    HorizontalDirection::Vertical => "down",
                    HorizontalDirection::North => "down north",
                    HorizontalDirection::Northeast => "down northeast",
                    HorizontalDirection::East => "down east",
                    HorizontalDirection::Southeast => "down southeast",
                    HorizontalDirection::South => "down south",
                    HorizontalDirection::Southwest => "down southwest",
                    HorizontalDirection::West => "down west",
                    HorizontalDirection::Northwest => "down northwest",
                }
            }
        }
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

