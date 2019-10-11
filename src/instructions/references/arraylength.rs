#[allow(non_camel_case_types)]
pub struct ARRAY_LENGTH {}

impl Debug for ARRAY_LENGTH {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}