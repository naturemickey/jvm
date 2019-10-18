pub enum Constant {
    Empty,
    Utf8(),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(),
    String(String),
    FieldRef(),
    MethodRef(),
    InterfaceMethodRef(),
    NameAndType(),
    MethodHandle(),
    MethodType(),
    Dynamic(),
    InvokeDynamic(),
    Module(),
    Package(),
}