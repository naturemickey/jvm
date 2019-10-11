#[allow(non_camel_case_types)]
pub struct ATHROW {}

impl Debug for ATHROW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}