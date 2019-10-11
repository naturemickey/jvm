#[allow(non_camel_case_types)]
pub struct AALOAD {}

impl Debug for AALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}