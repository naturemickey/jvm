#[allow(non_camel_case_types)]
pub struct I_RETURN {}

impl Debug for I_RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}