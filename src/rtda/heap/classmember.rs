struct ClassMember {
    access_flags: u16,
    name: String,
    descriptor: String,
    class: Arc<RwLock<Class>>,
}

impl ClassMember {
    fn new(class: Arc<RwLock<Class>>, member_info: &MemberInfo) -> ClassMember {
        let access_flags = member_info.access_flgs();
        let name = member_info.name().to_string();
        let descriptor = member_info.descriptor().to_string();
        Self { access_flags, name, descriptor, class }
    }

    fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }
    fn is_private(&self) -> bool {
        self.access_flags & ACC_PRIVATE != 0
    }
    fn is_protected(&self) -> bool {
        self.access_flags & ACC_PROTEDTED != 0
    }
    fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }
    fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }
    fn is_final(&self) -> bool { self.access_flags & ACC_FINAL != 0 }

    fn name(&self) -> &str {
        &self.name
    }
    fn descriptor(&self) -> &str {
        &self.descriptor
    }
    fn class(&self) -> Arc<RwLock<Class>> {
        self.class.clone()
    }

    fn is_accessible_to(&self, d: Arc<RwLock<Class>>) -> bool {
        if self.is_public() {
            true
        } else {
            let c = self.class();
            if self.is_protected() {
                d.read().unwrap().deref() == c.read().unwrap().deref() || d.read().unwrap().is_sub_class_of(&c.read().unwrap()) ||
                    c.read().unwrap().package_name().eq(d.read().unwrap().package_name())
            } else if !self.is_private() {
                c.read().unwrap().package_name().eq(d.read().unwrap().package_name())
            } else {
                d.read().unwrap().deref() == c.read().unwrap().deref()
            }
        }
    }
}