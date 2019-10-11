#[allow(non_camel_case_types)]
pub struct LALOAD {}

impl Debug for LALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}