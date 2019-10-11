#[allow(non_camel_case_types)]
pub struct INVOKE_DYNAMIC {}

impl Debug for INVOKE_DYNAMIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}