#[allow(non_camel_case_types)]
pub struct IMPDEP1 {}

impl Debug for IMPDEP1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}