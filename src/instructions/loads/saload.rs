#[allow(non_camel_case_types)]
pub struct SALOAD {}

impl Debug for SALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}