#[allow(non_camel_case_types)]
pub struct LDC {}

impl Debug for LDC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}