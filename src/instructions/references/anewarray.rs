#[allow(non_camel_case_types)]
pub struct ANEW_ARRAY {}

impl Debug for ANEW_ARRAY {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}