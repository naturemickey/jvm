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