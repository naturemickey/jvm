pub enum Constant {
    Empty,
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(ClassRef),
    String(String),
    FieldRef(FieldRef),
    MethodRef(MethodRef),
    InterfaceMethodRef(InterfaceMethodRef),
    NameAndType(),
    MethodHandle(),
    MethodType(),
    Dynamic(),
    InvokeDynamic(),
    Module(),
    Package(),
}

impl Constant {
    pub unsafe fn get_class_ref(&self) -> &ClassRef {
        match self {
            Constant::Class(class_ref) => class_ref,
            _ => panic!("unsafe called.")
        }
    }
    pub unsafe fn get_class_ref_mut(&mut self) -> &mut ClassRef {
        match self {
            Constant::Class(class_ref) => class_ref,
            _ => panic!("unsafe called.")
        }
    }

    pub unsafe fn get_field_ref(&self) -> &FieldRef {
        match self {
            Constant::FieldRef(field_ref) => field_ref,
            _ => panic!("unsafe called.")
        }
    }
    pub unsafe fn get_field_ref_mut(&mut self) -> &mut FieldRef {
        match self {
            Constant::FieldRef(field_ref) => field_ref,
            _ => panic!("unsafe called.")
        }
    }

    pub unsafe fn get_method_ref(&self) -> &MethodRef {
        match self {
            Constant::MethodRef(method_ref) => method_ref,
            _ => panic!("unsafe called.")
        }
    }
}