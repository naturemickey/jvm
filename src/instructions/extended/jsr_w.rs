#[allow(non_camel_case_types)]
pub struct JSR_W {}

impl Debug for JSR_W {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}