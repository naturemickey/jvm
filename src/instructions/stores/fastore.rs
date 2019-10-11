#[allow(non_camel_case_types)]
pub struct FASTORE {}

impl Debug for FASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}