use crate::error::{Error, Result};
use crate::cmd;

#[derive(PartialEq, Eq, Debug)]
pub struct TextInput<'input> {
    text: String,
    mode: Mode<'input>,
} 

impl<'input> TextInput<'input> {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
            mode: Mode::Unparsed
        }
    }

    fn parse(&'input mut self) -> Result<&'input Mode> {
        self.mode = Mode::parse(&self.text)?;
        Ok(&self.mode)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Mode<'input> {
    Unparsed,
    /// A command, either global or contextual.  
    /// E.g.: go east
    //Command(Command<'input>),
    /// Any further input is within the specified context.
    /// E.g.: .inventory
    Context(Context<'input>),
    /// The user's character speaks out loud in its local area.
    /// E.g.: 'Hello to everyone in this area
    Talk(Talk<'input>),
}

impl<'input> Mode<'input> {
    pub const CHAR_PERIOD: char = '.';
    pub const CHAR_SINGLE_QUOTE: char = '\'';

    pub fn parse(text: &'input str) -> Result<Self> {
        match text.chars().next() {
            Some(Self::CHAR_PERIOD) => Ok(Self::Context(Context::parse(text)?)),
            Some(Self::CHAR_SINGLE_QUOTE) => Ok(Self::Talk(Talk::parse(text)?)),
            _ => Err(Error::Generic("Dunno".to_string()))
        }
    }
}

pub trait Parser<'input> where Self: Sized {
    fn parse(text: &'input str) -> Result<Self>; 
}



#[derive(PartialEq, Eq, Debug)]
pub struct Context<'input> {
    name: &'input str,
}

impl<'input> Parser<'input> for Context<'input> {
    fn parse(text: &'input str) -> Result<Self> {
        let name = text.strip_prefix(Mode::CHAR_PERIOD)
            .ok_or_else(|| Error::InputParsing(text.to_owned()))?;

        Ok(Self {
            name
        })
    }
}

impl<'input> Context<'input> {
    pub fn new(name: &'input str) -> Self {
        Self {
            name
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Talk<'input> {
    message: &'input str
}

impl<'input> Parser<'input> for Talk<'input> {
    fn parse(text: &'input str) -> Result<Self> {
        let message = text.strip_prefix(Mode::CHAR_SINGLE_QUOTE)
            .ok_or_else(|| Error::InputParsing(text.to_owned()))?;

        Ok(Self {
            message
        })
    }
}

impl<'input> Talk<'input> {
    pub fn new(message: &'input str) -> Self {
        Self {
            message
        }
    }
}


pub struct Command {
    cmd: cmd::Cmd
}

impl<'input> Parser<'input> for Command {
    fn parse(text: &'input str) -> Result<Self> {
        Ok(Self {
            cmd: cmd::Cmd::parse(text)?
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_talk() {
        let tests: Vec<(&'static str, Result<Talk>)> = vec![
            ("'hello", Ok(Talk::new("hello"))),
            ("'hello there!", Ok(Talk::new("hello there!"))),
            ("command param", Err(Error::Test)),
            (".context", Err(Error::Test))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let actual = Talk::parse(input);
            assert_eq!(expected.is_ok(), actual.is_ok(), "Failed on: {input}");
            if expected.is_ok() {
                assert_eq!(expected.unwrap(), actual.unwrap());
            }
        }
    }

    #[test] 
    fn test_context() {
        let tests: Vec<(&'static str, Result<Context>)> = vec![
            (".inventory", Ok(Context::new("inventory"))),
            ("'hello there!", Err(Error::Test)),
            ("command style", Err(Error::Test))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let actual = Context::parse(input);
            assert_eq!(expected.is_ok(), actual.is_ok(), "Failed on: {input}");
            if expected.is_ok() {
                assert_eq!(expected.unwrap(), actual.unwrap());
            }
        }
    }

    #[test] 
    fn test_command() {
        let tests: Vec<(&'static str, Result<Command>)> = vec![
            //(".inventory", Ok(Context::new("inventory"))),
            //("'hello there!", Err(Error::Test)),
            //("go there", Ok(Command:))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let actual = Command::parse(input);
            assert_eq!(expected.is_ok(), actual.is_ok(), "Failed on: {input}");
            if expected.is_ok() {
                assert_eq!(expected.unwrap(), actual.unwrap());
            }
        }
    }


    #[test] 
    fn test_input_parse() {
        let tests: Vec<(&'static str, Result<Mode>)> = vec![
            (".inventory", Ok(Mode::Context(Context::new("inventory")))),
            ("'hello there!", Ok(Mode::Talk(Talk::new("hello there!")))),
            //("command style", Ok(Mode::(Context::new("inventory"))))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let mut text_input = TextInput::new(input.to_string());
            let actual = text_input.parse();
            assert_eq!(expected.is_ok(), actual.is_ok(), "Failed on: {input}");
            if expected.is_ok() {
                assert_eq!(&expected.unwrap(), actual.unwrap());
            }
        }
    }
}