#[allow(non_camel_case_types)]
pub struct PUT_FIELD {}

impl Debug for PUT_FIELD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}