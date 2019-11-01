pub struct Object {
    class: *const Class,
    fields: Box<Slots>,
}

//lazy_static! {
//static ref NULL: Object = Object { class: null(), fields: Slots::new(0) };
//}

impl PartialEq for Object {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        (self as *const Self) == (other as *const Self) || self.class == other.class || (self.fields.borrow() as *const Slots) == (other.fields.borrow() as *const Slots)
    }
}

impl Object {
    fn new(class: &Class) -> Arc<Object> {
        unimplemented!()
    }
    pub fn null() -> Rc<Object> {
        Rc::new(Object { class: null(), fields: Box::new(Slots::new(0)) })
    }
}

const OBJECT_CLASS_NAME: &str = "java/lang/Object";