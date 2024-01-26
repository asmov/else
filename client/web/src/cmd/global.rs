use clap;
use crate::cmd::*;

#[derive(clap::Parser)]
pub struct GoCmd {
    area: String
}

impl GoCmd {
    pub const NAME: &'static str = "go";
}