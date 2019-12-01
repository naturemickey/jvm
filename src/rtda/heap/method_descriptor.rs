struct MethodDescriptor {
    parameter_types: Vec<String>,
    return_type: String,
}

impl MethodDescriptor {
    fn new() -> Self {
        let parameter_types = Vec::new();
        let return_type = "".to_string();
        Self { parameter_types, return_type }
    }

    fn add_parameter_type(&mut self, t: String) {
        unimplemented!()
    }
}