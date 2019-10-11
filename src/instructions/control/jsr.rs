#[allow(non_camel_case_types)]
pub struct JSR {}

impl Debug for JSR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}