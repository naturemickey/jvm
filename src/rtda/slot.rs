pub enum Slot {
    Num(u32),
    Ref(Object),
}

impl Copy for Slot {}

impl Clone for Slot {
    fn clone(&self) -> Self {
        *self
    }
}

impl Debug for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Num(num) => write!(f, "Slot::Num({})", num),
            Self::Ref(num) => write!(f, "Slot::Ref()"),
        }
    }
}