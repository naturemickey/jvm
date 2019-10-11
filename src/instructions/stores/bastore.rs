#[allow(non_camel_case_types)]
pub struct BASTORE {}

impl Debug for BASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}