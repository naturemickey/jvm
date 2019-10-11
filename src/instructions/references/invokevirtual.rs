#[allow(non_camel_case_types)]
pub struct INVOKE_VIRTUAL {}

impl Debug for INVOKE_VIRTUAL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}