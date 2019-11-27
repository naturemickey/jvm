pub enum Slot {
    Num(u32),
    Ref(Arc<RwLock<Object>>),
}

impl Clone for Slot {
    fn clone(&self) -> Self {
        match self {
            Slot::Num(n) => Slot::Num(*n),
            Slot::Ref(rc_obj) => Slot::Ref(rc_obj.clone()),
        }
    }
}

impl Slot {}

impl Debug for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Num(num) => write!(f, "Slot::Num({})", num),
            Self::Ref(_num) => write!(f, "Slot::Ref()"),
        }
    }
}