#[allow(non_camel_case_types)]
pub struct NEW_ARRAY {}

impl Debug for NEW_ARRAY {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}