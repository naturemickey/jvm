#[allow(non_camel_case_types)]
pub struct SASTORE {}

impl Debug for SASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}