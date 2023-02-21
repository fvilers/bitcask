use std::fmt;

pub struct CustomPrompt;

impl fmt::Display for CustomPrompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "> ")
    }
}
