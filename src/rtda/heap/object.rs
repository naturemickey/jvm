pub struct Object {
    class: Option<Arc<RwLock<Class>>>,
    fields: Slots,
}

//lazy_static! {
//static ref NULL: Object = Object { class: null(), fields: Slots::new(0) };
//}

impl PartialEq for Object {
    // #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
            || (self.is_null() && other.is_null())
//        if self as *const Self == other as *const Self {
//            true
//        } else if self.class.is_none() && other.class.is_none() {
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
    fn new(class: Arc<RwLock<Class>>) -> Arc<RwLock<Object>> {
        let fields = Slots::new(class.read().unwrap().instance_slot_count);
        Arc::new(RwLock::new(Self { class: Some(class), fields }))
    }
    pub fn null() -> Arc<RwLock<Object>> {
        // todo
        Arc::new(RwLock::new(Object { class: None, fields: Slots::new(0) }))
    }
    pub fn fields_mut(&mut self) -> &mut Slots {
        &mut self.fields
    }
    pub fn is_instance_of(&self, class: &Class) -> bool {
        match &self.class {
            Some(c) => c.read().unwrap().is_assignable_from(class),
            None => true,
        }
    }

    pub fn is_null(&self) -> bool {
        self.class.is_none()
    }
}

const OBJECT_CLASS_NAME: &str = "java/lang/Object";