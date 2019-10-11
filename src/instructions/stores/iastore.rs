#[allow(non_camel_case_types)]
pub struct IASTORE {}

impl Debug for IASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}