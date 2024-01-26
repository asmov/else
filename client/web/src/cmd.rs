pub mod global;

use clap::Parser;
use shellwords;
use crate::{error::*, cmd::global::*};


pub enum Cmd {
    Go(GoCmd),
}

impl Cmd {
    pub fn parse(text: &str) -> Result<Self> {
        /*let end_pos = text.find(' ')
            .or_else(|| Some(text.len() - 1))
            .unwrap();
        let cmdname = &text[0..end_pos];*/

        let args = shellwords::split(text)
            .map_err(|e| Error::InputParsing(text.to_string()))?;

        let cmdname = args.get(0)
            .ok_or_else(|| Error::InputParsing(text.to_string()))?;
        
        let cmd = match cmdname.as_str() {
            GoCmd::NAME => GoCmd::try_parse_from(args).map_err(|e| Error::InputParsing(text.to_string()))?,
            _ => return Err(Error::UnknownCommand(text.to_owned()))
        };

        Ok(Self::Go(cmd))
    }
}