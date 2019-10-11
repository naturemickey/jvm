#[allow(non_camel_case_types)]
pub struct GET_STATIC {}

impl Debug for GET_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}