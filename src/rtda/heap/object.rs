pub struct Object {
    class: Option<Arc<Class>>,
    fields: Slots,
}

//lazy_static! {
//static ref NULL: Object = Object { class: null(), fields: Slots::new(0) };
//}

impl PartialEq for Object {
    // #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
//        if self as *const Self == other as *const Self {
//            true
//        } else if self.class == None && other.class == None {
//            true
//        } else {
//            let c1 = match &self.class {
//                Some(ac) => ac.clone(),
//                None => return false
//            };
//            let c2 = match &other.class {
//                Some(ac) => ac.clone(),
//                None => return false
//            };
//            c1.as_ref() as *const Class == c2.as_ref() as *const Class
//                || (self.fields.borrow() as *const Slots) == (other.fields.borrow() as *const Slots)
//        }
    }
}

impl Object {
    fn new(class: Arc<Class>) -> Arc<Object> {
        let fields = Slots::new(class.instance_slot_count);
        Arc::new(Self { class: Some(class), fields })
    }
    pub fn null() -> Arc<Object> {
        Arc::new(Object { class: None, fields: Slots::new(0) })
    }
    pub fn fields_mut(&mut self) -> &mut Slots {
        &mut self.fields
    }
    pub fn is_instance_of(&self, class: &Class) -> bool {
        match &self.class {
            Some(c) => c.is_assignable_from(class),
            None => true,
        }
    }
}

const OBJECT_CLASS_NAME: &str = "java/lang/Object";