#[allow(non_camel_case_types)]
pub struct A_RETURN {}

impl Debug for A_RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),  Error> {
        write!(f, "()")
    }
}