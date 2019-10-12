pub struct Object {}

pub const NULL: Object = Object {};

impl Copy for Object {}

impl Clone for Object {
    fn clone(&self) -> Self {
        *self
    }
}

impl PartialEq for Object {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        (self as *const Self) == (other as *const Self)
    }
}