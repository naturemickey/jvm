#[allow(non_camel_case_types)]
pub struct IALOAD {}

impl Debug for IALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}