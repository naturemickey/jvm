#[allow(non_camel_case_types)]
pub struct CALOAD {}

impl Debug for CALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}