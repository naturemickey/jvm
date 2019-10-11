#[allow(non_camel_case_types)]
pub struct GET_FIELD {}

impl Debug for GET_FIELD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}