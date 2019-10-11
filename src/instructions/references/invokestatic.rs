#[allow(non_camel_case_types)]
pub struct INVOKE_STATIC {}

impl Debug for INVOKE_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}