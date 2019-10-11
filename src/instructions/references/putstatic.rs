#[allow(non_camel_case_types)]
pub struct PUT_STATIC {}

impl Debug for PUT_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}