#[allow(non_camel_case_types)]
pub struct NEW {}

impl Debug for NEW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}