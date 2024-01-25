use std::{fmt::Display, path::PathBuf};

#[macro_export]
macro_rules!log {
    ($($arg:tt)*) => {
        println!($($arg)*);
    }
}

#[macro_export]
macro_rules!log_error {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
    }
}

pub fn data_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn certs_dir() -> PathBuf {
    data_dir().join("certs")
}

#[derive(Clone, Debug)]
pub enum Who {
    Client (usize, String),
    Zone (usize, String),
    World (usize, String)
}

impl Display for Who {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Who::Client(num, addr) => write!(f, "client ({}#{})", addr, num),
            Who::Zone(num, addr) => write!(f, "zone server ({}#{})", addr, num),
            Who::World(num, addr) => write!(f, "world server ({}#{})", addr, num)
        }
    }
}

