pub struct Object {}

pub const NULL: Object = Object {};

impl Copy for Object {}

impl Clone for Object {
    fn clone(&self) -> Self {
        *self
    }
}