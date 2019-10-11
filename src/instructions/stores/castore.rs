#[allow(non_camel_case_types)]
pub struct CASTORE {}

impl Debug for CASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}