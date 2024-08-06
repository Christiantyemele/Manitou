use std::fmt::{self, Display};

mod compat;
#[macro_use]
pub mod constant;
pub struct EnumParseError(());

impl Display for EnumParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse Key from string")
    }
}
