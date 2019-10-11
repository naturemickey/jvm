#[allow(non_camel_case_types)]
pub struct BREAK_POINT {}

impl Debug for BREAK_POINT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}