#[allow(non_camel_case_types)]
pub struct FALOAD {}

impl Debug for FALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}