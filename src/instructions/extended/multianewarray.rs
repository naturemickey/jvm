#[allow(non_camel_case_types)]
pub struct MULTI_ANEW_ARRAY {}

impl Debug for MULTI_ANEW_ARRAY {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}