#[allow(non_camel_case_types)]
pub struct DASTORE {}

impl Debug for DASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}