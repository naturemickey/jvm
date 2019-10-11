#[allow(non_camel_case_types)]
pub struct LASTORE {}
impl Debug for LASTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}