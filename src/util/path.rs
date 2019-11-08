#[cfg(windows)]
pub const SEPARATOR: char = ':';

#[cfg(not(windows))]
pub const SEPARATOR: char = ';';

#[cfg(windows)]
pub const SEPARATOR_STR: &str = ";";

#[cfg(not(windows))]
pub const SEPARATOR_STR: &str = ":";