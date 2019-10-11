#[allow(non_camel_case_types)]
pub struct CHECK_CAST {}

impl Debug for CHECK_CAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}