#[allow(non_camel_case_types)]
pub struct MONITOR_EXIT {}

impl Debug for MONITOR_EXIT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}