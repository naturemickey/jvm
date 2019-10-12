
struct Class<'a> {
    access_flags : u16,
    name : String,
    super_class_name : String,
    interface_names : Vec<String>,
    constant_pool : Arc<ConstantPool>,
    fields: Vec<Field>,
    methods : Vec<Method>,
    loader : &'a ClassLoader,
    super_class : &'a Class<'a>,
    interfaces : Vec<&'a Class<'a>>,
    instance_slot_count:usize,
    static_slot_count:usize,
    static_vars : &'a Slots,
}