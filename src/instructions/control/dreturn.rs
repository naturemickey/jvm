#[allow(non_camel_case_types)]
pub struct D_RETURN {}

impl Debug for D_RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}