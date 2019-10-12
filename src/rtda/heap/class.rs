struct Class<'a> {
    access_flags: u16,
    name: &'a str,
    super_class_name: &'a str,
    interface_names: Vec<&'a str>,
    constant_pool: Arc<ConstantPool>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    loader: &'a ClassLoader,
    super_class: &'a Class<'a>,
    interfaces: Vec<&'a Class<'a>>,
    instance_slot_count: usize,
    static_slot_count: usize,
    static_vars: &'a Slots,
}

impl<'a> Class<'a> {
    pub fn new(cf:&'a ClassFile) -> Box<Class<'a>> {
        let access_flags = cf.access_flags();
        let name = cf.class_name();
        let super_class_name = cf.super_class_name();
        let interface_names = cf.interface_names();
        let fields = Field::new();
    }
}