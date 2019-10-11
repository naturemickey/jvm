#[allow(non_camel_case_types)]
pub struct RET {}

impl Debug for RET {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}