#[allow(non_camel_case_types)]
pub struct RETURN {}

impl Debug for RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}