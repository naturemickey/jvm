#[allow(non_camel_case_types)]
pub struct BALOAD {}

impl Debug for BALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}