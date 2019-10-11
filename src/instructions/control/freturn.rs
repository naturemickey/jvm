#[allow(non_camel_case_types)]
pub struct F_RETURN {}

impl Debug for F_RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}