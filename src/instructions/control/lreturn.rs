#[allow(non_camel_case_types)]
pub struct L_RETURN {}

impl Debug for L_RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}