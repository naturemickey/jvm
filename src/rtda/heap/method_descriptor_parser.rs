struct MethodDescriptorParser {
    raw: String,
    offset: usize,
    parsed: MethodDescriptor,
}

impl MethodDescriptorParser {
    fn new(descriptor: &str) -> Self {
        Self { raw: descriptor.to_string(), offset: 0 ,parsed:MethodDescriptor::new()}
    }

    fn parse(descriptor: &str) -> MethodDescriptor {
        let mut parser = Self::new(descriptor);
        parser.start_params();
        parser.parse_param_types();
        parser.end_params();
        parser.parse_return_type();
        parser.finish();
        parser.parsed
    }

    fn start_params(&mut self) {
        unimplemented!()
    }

    fn end_params(&mut self) {
        unimplemented!()
    }

    fn finish(&mut self) {
        unimplemented!()
    }

    fn cause_panic(&mut self) {
        unimplemented!()
    }

    fn read_u8(&mut self) {
        unimplemented!()
    }

    fn unread_u8(&mut self) {
        unimplemented!()
    }

    fn parse_param_types(&mut self) {
        unimplemented!()
    }

    fn parse_return_type(&mut self) {
        unimplemented!()
    }

    fn parse_field_type(&mut self) -> String {
        unimplemented!()
    }

    fn parse_object_type(&mut self) -> String {
        unimplemented!()
    }

    fn parse_array_type(&mut self) -> String {
        unimplemented!()
    }
}