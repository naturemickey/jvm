#[allow(non_camel_case_types)]
pub struct INVOKE_INTERFACE {}

impl Debug for INVOKE_INTERFACE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}