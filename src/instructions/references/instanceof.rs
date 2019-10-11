#[allow(non_camel_case_types)]
pub struct INSTANCE_OF {}

impl Debug for INSTANCE_OF {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}