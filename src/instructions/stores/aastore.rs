#[allow(non_camel_case_types)]
pub struct AASTORE {}

impl Debug for AASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}