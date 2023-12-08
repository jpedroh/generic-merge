use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MergeError {
    MergingTerminalWithNonTerminal,
    InvalidMatchingConfiguration(bool, bool, bool, bool, bool),
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MergeError::MergingTerminalWithNonTerminal => {
                write!(f, "Merging terminal with non-terminal")
            }
            MergeError::InvalidMatchingConfiguration(a, b, c, d, e) => write!(
                f,
                "Invalid matching configuration: {}, {}, {}, {}, {}",
                a, b, c, d, e
            ),
        }
    }
}

impl error::Error for MergeError {}
