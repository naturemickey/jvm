#[allow(non_camel_case_types)]
pub struct MONITOR_ENTER {}

impl Debug for MONITOR_ENTER {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), dyn Error> {
        write!(f, "()")
    }
}