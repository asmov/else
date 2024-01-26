use crate::error::{Error, Result};
use crate::cmd;

#[derive(PartialEq, Eq, Debug)]
pub struct TextInput {
    text: String,
    tokens: Vec<String>,
} 

impl TextInput {
    pub fn parse(text: String) -> Result<Self> {
        let tokens = split_arguments(&text)?;
        Ok(Self {
            text,
            tokens,
        })
    }

    pub fn args(&self) -> &Vec<String> {
        &self.tokens
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

trait Parser where Self: Sized {
    fn parse(input: TextInput) -> Result<Self>; 
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParsedInput {
    /// A command, either global or contextual.  
    /// E.g.: go east
    Command(Command),
    /// Any further input is within the specified context.
    /// E.g.: .inventory
    Context(Context),
    /// The user's character speaks out loud in its local area.
    /// E.g.: 'Hello to everyone in this area
    Talk(Talk),
}

impl ParsedInput {
    pub const CHAR_PERIOD: char = '.';
    pub const CHAR_SINGLE_QUOTE: char = '\'';

    pub fn parse(text: String) -> Result<Self> {
        let input = TextInput::parse(text)?;
        let tokens = input.args();

        let first_char = tokens.get(0)
            .ok_or_else(|| Error::GenericInputParsing(input.text().to_string()))?
            .chars()
            .next().unwrap();

        let parsed = match first_char {
            Self::CHAR_PERIOD => Self::Context(Context::parse(input)?),
            Self::CHAR_SINGLE_QUOTE => Self::Talk(Talk::parse(input)?),
            _ => Self::Command(Command::parse(input)?)
        };

        Ok(parsed)
    }
}



#[derive(PartialEq, Eq, Debug)]
pub struct Context {
    name: String
}

impl Parser for Context {
    fn parse(input: TextInput) -> Result<Self> {
        let name = input.text()
            .strip_prefix(ParsedInput::CHAR_PERIOD)
            .ok_or_else(|| Error::GenericInputParsing(input.text().to_string()))?
            .to_owned();
        
        Ok(Self {
            name
        })
    }
}

impl Context {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Talk {
    message: String
}

impl Parser for Talk {
    fn parse(input: TextInput) -> Result<Self> {
        let message = input.text()
            .strip_prefix(ParsedInput::CHAR_SINGLE_QUOTE)
            .ok_or_else(|| Error::GenericInputParsing(input.text().to_string()))?
            .to_owned();

        Ok(Self {
            message
        })
    }
}

impl Talk {
    pub fn new(message: String) -> Self {
        Self {
            message 
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    cmd: cmd::Cmd
}

impl Parser for Command {
    fn parse(input: TextInput) -> Result<Self> {
        Ok(Self {
            cmd: cmd::Cmd::parse(&input)?
        })
    }
}

impl Command {
    pub fn new(cmd: cmd::Cmd) -> Self {
        Self {
            cmd
        }
    }
}

/// Processes a raw command-line string and splits each word or double-quoted section into an argument.  
/// Examples:
/// - `go there` -> ["go", "there"]
/// - `go "there again"` -> ["go", "there again"]
/// - `greet "There are some who call me ... \"Tim\"."` -> ["greet", "There are some who call me ... \"Tim\"."]
fn split_arguments(text: &str) -> Result<Vec<String>> {
    const TRIMS: [char;2] = [' ', '"'];
    const REPLACE_ESC_QUOTE: (&'static str, &'static str) = ("\\\"", "\"");
    let text = text.trim();
    let mut arguments: Vec<String> = Vec::new();
    let mut quote_open = false;
    let mut escaped = false;
    let mut start_pos: usize = 0;

    for (char_index, chr) in text.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match chr {
            '"' => {
                if quote_open {
                    quote_open = false;
                    let arg = &text[start_pos..char_index]
                        .trim_matches(TRIMS)
                        .replace(REPLACE_ESC_QUOTE.0, REPLACE_ESC_QUOTE.1);
                    if !arg.is_empty() {
                        arguments.push(arg.to_string());
                    }
                    start_pos = char_index;
                } else {
                    quote_open = true;
                    start_pos = char_index;
                }
            },
            ' ' => {
                if !quote_open {
                    let arg = &text[start_pos..char_index]
                        .trim_matches(TRIMS)
                        .replace(REPLACE_ESC_QUOTE.0, REPLACE_ESC_QUOTE.1);
                    if !arg.is_empty() {
                        arguments.push(arg.to_string());
                    }
                    start_pos = char_index;
                }
            },
            '\\' => {
                escaped = true;
            },
            _ => {
            }
        }
    }

    if quote_open {
        return Err(Error::GenericInputParsing(text.to_string()))
    }

    let arg = &text[start_pos..]
        .trim_matches(TRIMS)
        .replace(REPLACE_ESC_QUOTE.0, REPLACE_ESC_QUOTE.1);
    if !arg.is_empty() {
        arguments.push(arg.to_string());
    }

    Ok(arguments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use elsezone_model::s;

    #[test] 
    fn test_talk() {
        let tests: Vec<(&'static str, Result<Talk>)> = vec![
            ("'hello", Ok(Talk::new(s!("hello")))),
            ("'hello there!", Ok(Talk::new(s!("hello there!")))),
            ("command param", Err(Error::Test)),
            (".context", Err(Error::Test))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let actual = Talk::parse(TextInput::parse(input.to_string()).unwrap());
            assert_eq!(expected.is_ok(), actual.is_ok(), "Failed on: {input}");
            if expected.is_ok() {
                assert_eq!(expected.unwrap(), actual.unwrap());
            }
        }
    }

    #[test] 
    fn test_context() {
        let tests: Vec<(&'static str, Result<Context>)> = vec![
            (".inventory", Ok(Context::new(s!("inventory")))),
            ("'hello there!", Err(Error::Test)),
            ("command style", Err(Error::Test))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let actual = Context::parse(TextInput::parse(input.to_string()).unwrap());
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
            ("go there", Ok(Command::new(cmd::Cmd::Go(cmd::global::GoCmd::new(s!("there")))))),
            ("go", Err(Error::Test))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let actual = Command::parse(TextInput::parse(input.to_string()).unwrap());
            assert_eq!(expected.is_ok(), actual.is_ok(), "Failed on: {input}");
            if expected.is_ok() {
                assert_eq!(expected.unwrap(), actual.unwrap());
            } else {
                dbg!(actual.err().unwrap());
            }
        }
    }


    #[test] 
    fn test_input_parse() {
        let tests: Vec<(&'static str, Result<ParsedInput>)> = vec![
            (".inventory", Ok(ParsedInput::Context(Context::new(s!("inventory"))))),
            ("'hello there!", Ok(ParsedInput::Talk(Talk::new(s!("hello there!"))))),
            ("go there", Ok(ParsedInput::Command(Command::new(cmd::Cmd::Go(cmd::global::GoCmd::new(s!("there")))))))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let actual = ParsedInput::parse(input.to_string());
            assert_eq!(expected.is_ok(), actual.is_ok(), "Failed on: {input}");
            if expected.is_ok() {
                assert_eq!(expected.unwrap(), actual.unwrap());
            }
        }
    }

    #[test] 
    fn test_split_arguments() {
        let tests: Vec<(&'static str, Result<Vec<&'static str>>)> = vec![
            ("go", Ok(vec!["go"])),
            ("go there", Ok(vec!["go", "there"])),
            ("go \"there again\"", Ok(vec!["go", "there again"])),
            ("go \"there\" \"again\"", Ok(vec!["go", "there", "again"])),
            ("go --there \"again\"", Ok(vec!["go", "--there", "again"])),
            ("greet \"There are some who call me ... \\\"Tim\\\".\"",
                Ok(vec!["greet", "There are some who call me ... \"Tim\"."]))
        ];

        for test in tests {
            let input = test.0;
            let expected = test.1;
            let actual = split_arguments(input);
            assert_eq!(expected.is_ok(), actual.is_ok(), "Failed on: {input}");
            if expected.is_ok() {
                assert_eq!(expected.unwrap(), actual.unwrap());
            } else {
                dbg!(actual.err().unwrap());
            }
        }
    }


}