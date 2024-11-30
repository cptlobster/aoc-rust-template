/// Common functions used by all days
use std::fs;

/// Read the contents of an input file to a string
pub(crate) fn read_input(name: &str) -> String {
    fs::read_to_string(name).expect("Unable to read file {file}")
}
