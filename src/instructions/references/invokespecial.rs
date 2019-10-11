#[allow(non_camel_case_types)]
pub struct INVOKE_SPECIAL {}

impl Debug for INVOKE_SPECIAL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}