use serde;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
    UpNorth,
    UpEast,
    UpSouth,
    UpWest,
    DownNorth,
    DownEast,
    DownSouth,
    DownWest
}

