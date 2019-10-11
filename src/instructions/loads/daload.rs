#[allow(non_camel_case_types)]
pub struct DALOAD {}

impl Debug for DALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}