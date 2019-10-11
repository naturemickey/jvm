#[allow(non_camel_case_types)]
pub struct IMPDEP2 {}

impl Debug for IMPDEP2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}